use anyhow::Result;
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use prosettings_tui::cli;
use prosettings_tui::scraper::player_slug;
use prosettings_tui::ui::{search_player, App};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use std::env;
use std::io::Write;
use std::process::{self, Command};
use std::thread;
use std::time::Duration;

fn copy_to_clipboard(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    let copy = |cmd: &str, args: &[&str]| -> Option<bool> {
        let mut child = Command::new(cmd)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .ok()?;
        if let Some(stdin) = &mut child.stdin {
            let _ = stdin.write_all(text.as_bytes());
        }
        child.wait().ok().map(|status| status.success())
    };
    copy("wl-copy", &[])
        .or_else(|| copy("xclip", &["-selection", "clipboard"]))
        .or_else(|| copy("xsel", &["--clipboard", "--input"]))
        .unwrap_or(false)
}

fn copy_import_code(app: &App) {
    if let Some(code) = app.get_import_code() {
        if !copy_to_clipboard(&code) {
            app.show_toast("Copy failed");
            return;
        }

        app.set_copied();
        app.show_toast("Import code copied");

        let app_clone = app.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            app_clone.clear_copied();
        });
    } else {
        app.show_toast("Copy failed");
    }
}

fn contains(rect: Rect, column: u16, row: u16) -> bool {
    column >= rect.x && column < rect.x + rect.width && row >= rect.y && row < rect.y + rect.height
}

fn scroll_if_player_loaded(app: &App, code: KeyCode) -> bool {
    if !app.has_player_data() {
        return false;
    }

    match code {
        KeyCode::Up => app.scroll_up(),
        KeyCode::Down => app.scroll_down(),
        KeyCode::PageUp => app.scroll_page_up(),
        KeyCode::PageDown => app.scroll_page_down(),
        KeyCode::Home => app.scroll_top(),
        KeyCode::End => app.scroll_bottom(),
        _ => return false,
    }

    true
}

fn scroll_mouse_if_player_loaded(app: &App, kind: MouseEventKind) -> bool {
    if !app.has_player_data() {
        return false;
    }

    match kind {
        MouseEventKind::ScrollUp => app.scroll_up(),
        MouseEventKind::ScrollDown => app.scroll_down(),
        _ => return false,
    }

    true
}

fn handle_escape(app: &App) -> bool {
    if app.is_picker_open() {
        app.close_theme_picker();
        return false;
    }

    if app.cancel_search() {
        return false;
    }

    if app.is_on_landing() {
        return true;
    }

    app.go_to_landing();
    false
}

fn submit_search(app: &App) {
    let (query, is_loading) = {
        let state = app.state.lock().unwrap();
        (state.search_query.clone(), state.is_loading)
    };

    if !query.is_empty() && !is_loading {
        app.search(&query);
    }
}

fn handle_loading_key(app: &App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => return handle_escape(app),
        KeyCode::Enter => {}
        KeyCode::Backspace => {
            let mut state = app.state.lock().unwrap();
            state.search_query.pop();
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            let mut state = app.state.lock().unwrap();
            state.search_query.push(c);
        }
        _ => {}
    }

    false
}

fn handle_character(app: &App, c: char) {
    let is_search_empty = {
        let state = app.state.lock().unwrap();
        state.search_query.is_empty()
    };

    if is_search_empty {
        match c {
            't' | 'T' => app.open_theme_picker(),
            other => {
                let mut state = app.state.lock().unwrap();
                state.search_query.push(other);
            }
        }
    } else {
        let mut state = app.state.lock().unwrap();
        state.search_query.push(c);
    }
}

fn handle_picker_key(app: &App, key: KeyEvent) {
    let action = app.handle_theme_selector_key(key);
    if action.is_none() && key.code == KeyCode::Esc {
        app.close_theme_picker();
    }
}

fn handle_landing_key(app: &App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => true,
        KeyCode::Enter => {
            if !app.is_on_landing() {
                submit_search(app);
                return false;
            }
            let query = {
                let state = app.state.lock().unwrap();
                state.search_query.clone()
            };
            app.leave_landing();
            if !query.is_empty() {
                app.search(&query);
            }
            false
        }
        KeyCode::Backspace => {
            let mut state = app.state.lock().unwrap();
            state.search_query.pop();
            false
        }
        KeyCode::Char('t') | KeyCode::Char('T')
            if key.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            app.open_theme_picker();
            false
        }
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            let mut state = app.state.lock().unwrap();
            state.search_query.push(c);
            false
        }
        _ => false,
    }
}

fn handle_key(app: &App, key: KeyEvent) -> bool {
    if app.is_picker_open() {
        handle_picker_key(app, key);
        return false;
    }

    if app.is_on_landing() {
        return handle_landing_key(app, key);
    }

    let is_loading = {
        let state = app.state.lock().unwrap();
        state.is_loading
    };
    if is_loading {
        return handle_loading_key(app, key);
    }

    match key.code {
        KeyCode::Char('t') | KeyCode::Char('T')
            if key.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            app.open_theme_picker();
        }
        KeyCode::Char('y') | KeyCode::Char('Y')
            if key.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            copy_import_code(app);
        }
        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.prev_tab(),
        KeyCode::Esc => return handle_escape(app),
        KeyCode::Up
        | KeyCode::Down
        | KeyCode::PageUp
        | KeyCode::PageDown
        | KeyCode::Home
        | KeyCode::End => {
            scroll_if_player_loaded(app, key.code);
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            let search_empty = {
                let state = app.state.lock().unwrap();
                state.search_query.is_empty()
            };
            if search_empty {
                return true;
            }
            let mut state = app.state.lock().unwrap();
            state.search_query.push('q');
        }
        KeyCode::Enter => submit_search(app),
        KeyCode::Backspace => {
            let mut state = app.state.lock().unwrap();
            state.search_query.pop();
        }
        KeyCode::Char(c) => handle_character(app, c),
        _ => {}
    }

    false
}

fn handle_event(app: &App, event: Event) -> bool {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => handle_key(app, key),
        Event::Mouse(mouse) => {
            if !app.is_picker_open() {
                let state = app.state.lock().unwrap();
                if state.is_loading {
                    return false;
                }
                let in_settings = state
                    .settings_rect
                    .map(|r| contains(r, mouse.column, mouse.row))
                    .unwrap_or(false);
                drop(state);

                if in_settings {
                    scroll_mouse_if_player_loaded(app, mouse.kind);
                }
            }
            false
        }
        _ => false,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match cli::parse_args(&args) {
        Err(help_text) => {
            eprintln!("{}", help_text);
            process::exit(0);
        }
        Ok(cli::Action::Query { slug, output }) => {
            let mut result: Result<_, String> = Err("unknown error".into());
            cli::with_spinner(&slug, || {
                result = search_player(&slug)
                    .map_err(|e| format!("failed to fetch player data: {}", e));
            });

            match result {
                Ok(player) => output.print(&player),
                Err(e) => {
                    eprintln!("error: {}", e);
                    process::exit(1);
                }
            }
        }
        Ok(cli::Action::Tui { pre_search }) => {
            if let Err(e) = run_tui(pre_search) {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}

fn run_tui(pre_search: Option<String>) -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();

    if let Some(name) = pre_search {
        if let Some(slug) = player_slug(&name) {
            // Set loading state immediately so TUI shows spinner right away
            let request_id = {
                let mut state = app.state.lock().unwrap();
                state.leave_landing();
                state.search_query = name;
                state.begin_search(&slug).map(|(rid, _)| rid)
            };

            if let Some(request_id) = request_id {
                let app_clone = app.clone();
                thread::spawn(move || {
                    let result = search_player(&slug);
                    let mut state = app_clone.state.lock().unwrap();
                    state.finish_search(request_id, result);
                });
            }
        }
    }

    let result = run_tui_loop(&mut terminal, app);

    disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    result
}

fn run_tui_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: App,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            app.render(f);
        })?;

        app.tick_frame();

        if crossterm::event::poll(Duration::from_millis(100))?
            && handle_event(&app, crossterm::event::read()?)
        {
            break;
        }
    }

    Ok(())
}
