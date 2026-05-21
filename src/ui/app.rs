use crate::models::{PlayerData, Tab};
use crate::scraper::{
    enrich_with_totalcsgo_crosshair, fetch_totalcsgo_crosshair, parse_player_data, player_slug,
    run_node_scraper,
};
use crate::theme::ThemeManager;
use opaline::widgets::{ThemeSelectorAction, ThemeSelectorState};
use ratatui::layout::Rect;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub struct AppState {
    pub player_data: Option<PlayerData>,
    pub current_tab: Tab,
    pub search_query: String,
    pub is_loading: bool,
    pub loading_query: Option<String>,
    pub request_id: u64,
    pub error_message: Option<String>,
    pub copied: bool,
    pub theme_manager: ThemeManager,
    pub animation_started_at: Instant,
    pub toast_visible: bool,
    pub toast_msg: String,
    pub toast_shown_at: Option<Instant>,
    pub scroll_offset: usize,
    pub visible_rows: usize,
    pub theme_selector: Option<ThemeSelectorState>,
    pub settings_rect: Option<Rect>,
    pub on_landing: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            player_data: None,
            current_tab: Tab::Mouse,
            search_query: String::new(),
            is_loading: false,
            loading_query: None,
            request_id: 0,
            error_message: None,
            copied: false,
            theme_manager: ThemeManager::new(),
            animation_started_at: Instant::now(),
            toast_visible: false,
            toast_msg: String::new(),
            toast_shown_at: None,
            scroll_offset: 0,
            visible_rows: 0,
            theme_selector: None,
            settings_rect: None,
            on_landing: true,
        }
    }

    pub fn go_to_landing(&mut self) {
        self.cancel_search();
        self.player_data = None;
        self.error_message = None;
        self.search_query.clear();
        self.copied = false;
        self.scroll_offset = 0;
        self.current_tab = Tab::Mouse;
        self.on_landing = true;
    }

    pub fn leave_landing(&mut self) {
        self.on_landing = false;
    }

    pub fn begin_search(&mut self, query: &str) -> Option<(u64, String)> {
        if self.is_loading {
            return None;
        }

        let slug = player_slug(query)?;

        self.is_loading = true;
        self.loading_query = Some(query.trim().to_string());
        self.request_id = self.request_id.wrapping_add(1);
        self.error_message = None;
        self.copied = false;
        self.scroll_offset = 0;

        Some((self.request_id, slug))
    }

    pub fn finish_search(&mut self, request_id: u64, result: Result<PlayerData, String>) -> bool {
        if !self.is_loading || self.request_id != request_id {
            return false;
        }

        self.is_loading = false;
        self.loading_query = None;

        match result {
            Ok(player) => {
                self.player_data = Some(player);
                self.error_message = None;
            }
            Err(message) => {
                self.player_data = None;
                self.error_message = Some(message);
            }
        }

        true
    }

    pub fn cancel_search(&mut self) -> bool {
        if !self.is_loading {
            return false;
        }

        self.is_loading = false;
        self.loading_query = None;
        self.request_id = self.request_id.wrapping_add(1);

        true
    }

    pub fn next_tab(&mut self) {
        self.current_tab.next();
        self.scroll_offset = 0;
    }

    pub fn prev_tab(&mut self) {
        self.current_tab.prev();
        self.scroll_offset = 0;
    }

    pub fn get_import_code(&self) -> Option<String> {
        self.player_data
            .as_ref()
            .and_then(|p| p.data.crosshair.import_code.clone())
            .filter(|code| !code.trim().is_empty())
    }

    pub fn clear_copied(&mut self) {
        self.copied = false;
    }

    pub fn set_copied(&mut self) {
        self.copied = true;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    pub fn scroll_by(&mut self, delta: isize) {
        if delta.is_negative() {
            self.scroll_offset = self.scroll_offset.saturating_sub(delta.unsigned_abs());
        } else {
            self.scroll_offset = self.scroll_offset.saturating_add(delta as usize);
        }
    }

    pub fn scroll_page_up(&mut self) {
        let page = self.visible_rows.max(1);
        self.scroll_offset = self.scroll_offset.saturating_sub(page);
    }

    pub fn scroll_page_down(&mut self) {
        let page = self.visible_rows.max(1);
        self.scroll_offset = self.scroll_offset.saturating_add(page);
    }

    pub fn scroll_top(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn scroll_bottom(&mut self) {
        self.scroll_offset = usize::MAX;
    }

    pub fn clamp_scroll_to(&mut self, max_scroll: usize) {
        self.scroll_offset = self.scroll_offset.min(max_scroll);
    }

    pub fn clear_player(&mut self) {
        self.cancel_search();
        self.player_data = None;
        self.copied = false;
        self.scroll_offset = 0;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct App {
    pub state: Arc<Mutex<AppState>>,
}

impl Clone for App {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(AppState::new())),
        }
    }

    pub fn search(&self, query: &str) -> bool {
        let Some((request_id, slug)) = ({
            let mut state = self.state.lock().unwrap();
            state.begin_search(query)
        }) else {
            return false;
        };

        let app = self.clone();
        thread::spawn(move || {
            let result = search_player(&slug);
            let mut state = app.state.lock().unwrap();
            state.finish_search(request_id, result);
        });

        true
    }

    pub fn cancel_search(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        state.cancel_search()
    }

    pub fn next_tab(&self) {
        let mut state = self.state.lock().unwrap();
        state.next_tab();
    }

    pub fn prev_tab(&self) {
        let mut state = self.state.lock().unwrap();
        state.prev_tab();
    }

    pub fn has_player_data(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.player_data.is_some()
    }

    pub fn set_copied(&self) {
        let mut state = self.state.lock().unwrap();
        state.set_copied();
    }

    pub fn get_import_code(&self) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.get_import_code()
    }

    pub fn clear_copied(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear_copied();
    }

    pub fn show_toast(&self, msg: &str) {
        let mut state = self.state.lock().unwrap();
        state.toast_visible = true;
        state.toast_msg = msg.to_string();
        state.toast_shown_at = Some(Instant::now());
    }

    pub fn tick_frame(&self) {
        let mut state = self.state.lock().unwrap();
        if state.toast_visible {
            if let Some(t) = state.toast_shown_at {
                if t.elapsed().as_secs() >= 2 {
                    state.toast_visible = false;
                }
            }
        }
    }

    pub fn scroll_up(&self) {
        let mut state = self.state.lock().unwrap();
        state.scroll_up();
    }

    pub fn scroll_down(&self) {
        let mut state = self.state.lock().unwrap();
        state.scroll_down();
    }

    pub fn scroll_by(&self, delta: isize) {
        let mut state = self.state.lock().unwrap();
        state.scroll_by(delta);
    }

    pub fn scroll_page_up(&self) {
        let mut state = self.state.lock().unwrap();
        state.scroll_page_up();
    }

    pub fn scroll_page_down(&self) {
        let mut state = self.state.lock().unwrap();
        state.scroll_page_down();
    }

    pub fn scroll_top(&self) {
        let mut state = self.state.lock().unwrap();
        state.scroll_top();
    }

    pub fn scroll_bottom(&self) {
        let mut state = self.state.lock().unwrap();
        state.scroll_bottom();
    }

    pub fn clear_player_data(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear_player();
    }

    pub fn go_to_landing(&self) {
        let mut state = self.state.lock().unwrap();
        state.go_to_landing();
    }

    pub fn leave_landing(&self) {
        let mut state = self.state.lock().unwrap();
        state.leave_landing();
    }

    pub fn is_on_landing(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.on_landing
    }

    pub fn is_picker_open(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.theme_selector.is_some()
    }

    pub fn open_theme_picker(&self) {
        let mut state = self.state.lock().unwrap();
        state.theme_selector = Some(ThemeSelectorState::with_current_selected());
    }

    pub fn close_theme_picker(&self) {
        let mut state = self.state.lock().unwrap();
        state.theme_selector = None;
    }

    pub fn handle_theme_selector_key(
        &self,
        key: crossterm::event::KeyEvent,
    ) -> Option<ThemeSelectorAction> {
        let mut state = self.state.lock().unwrap();
        if let Some(ref mut selector) = state.theme_selector {
            let action = selector.handle_key(key);
            match action {
                ThemeSelectorAction::Select(ref name) => {
                    state.theme_manager.select_theme(name);
                    state.theme_selector = None;
                }
                ThemeSelectorAction::Cancel => {
                    state.theme_selector = None;
                }
                _ => {}
            }
            Some(action)
        } else {
            None
        }
    }
}

pub fn search_player(slug: &str) -> Result<PlayerData, String> {
    let json = run_node_scraper(slug).map_err(|e| e.user_message())?;
    let mut player = parse_player_data(&json, slug).map_err(|e| e.user_message())?;

    if let Ok(Some(totalcsgo)) = fetch_totalcsgo_crosshair(slug) {
        enrich_with_totalcsgo_crosshair(&mut player, totalcsgo);
    }

    Ok(player)
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;
    use crate::models::{CrosshairSettings, PlayerData, PlayerSettings};

    fn player_with_crosshair(crosshair: CrosshairSettings) -> PlayerData {
        PlayerData {
            player: "test".to_string(),
            slug: "test".to_string(),
            url: String::new(),
            data: PlayerSettings {
                crosshair,
                ..PlayerSettings::default()
            },
        }
    }

    #[test]
    fn get_import_code_returns_import_code_only() {
        let mut state = AppState::new();
        state.player_data = Some(player_with_crosshair(CrosshairSettings {
            import_code: Some("CSGO-abcde-fghij-klmno-pqrst-uvwxy".to_string()),
            command: Some("cl_crosshairsize 2; cl_crosshairgap -3".to_string()),
            ..CrosshairSettings::default()
        }));

        assert_eq!(
            state.get_import_code(),
            Some("CSGO-abcde-fghij-klmno-pqrst-uvwxy".to_string())
        );
    }

    #[test]
    fn get_import_code_filters_missing_and_empty_values() {
        let mut state = AppState::new();
        state.player_data = Some(player_with_crosshair(CrosshairSettings {
            import_code: Some("   ".to_string()),
            ..CrosshairSettings::default()
        }));

        assert_eq!(state.get_import_code(), None);
    }

    #[test]
    fn copied_state_sets_and_clears() {
        let mut state = AppState::new();

        state.set_copied();
        assert!(state.copied);

        state.clear_copied();
        assert!(!state.copied);
    }

    #[test]
    fn begin_search_sets_loading_and_resets_transient_state() {
        let mut state = AppState::new();
        state.error_message = Some("old error".to_string());
        state.copied = true;
        state.scroll_offset = 12;

        let request = state.begin_search("  XANTARES  ");

        assert_eq!(request, Some((1, "xantares".to_string())));
        assert!(state.is_loading);
        assert_eq!(state.loading_query.as_deref(), Some("XANTARES"));
        assert_eq!(state.error_message, None);
        assert!(!state.copied);
        assert_eq!(state.scroll_offset, 0);
    }

    #[test]
    fn finish_search_applies_successful_current_result() {
        let mut state = AppState::new();
        let (request_id, _) = state.begin_search("test").unwrap();
        let player = player_with_crosshair(CrosshairSettings::default());

        assert!(state.finish_search(request_id, Ok(player)));

        assert!(!state.is_loading);
        assert_eq!(state.loading_query, None);
        assert_eq!(
            state.player_data.as_ref().map(|p| p.player.as_str()),
            Some("test")
        );
        assert_eq!(state.error_message, None);
    }

    #[test]
    fn finish_search_applies_failed_current_result() {
        let mut state = AppState::new();
        let (request_id, _) = state.begin_search("test").unwrap();
        state.player_data = Some(player_with_crosshair(CrosshairSettings::default()));

        assert!(state.finish_search(request_id, Err("not found".to_string())));

        assert!(!state.is_loading);
        assert_eq!(state.loading_query, None);
        assert!(state.player_data.is_none());
        assert_eq!(state.error_message.as_deref(), Some("not found"));
    }

    #[test]
    fn finish_search_ignores_stale_request_id() {
        let mut state = AppState::new();
        let (old_request_id, _) = state.begin_search("old").unwrap();
        state.cancel_search();
        let (current_request_id, _) = state.begin_search("current").unwrap();

        assert!(!state.finish_search(
            old_request_id,
            Ok(player_with_crosshair(CrosshairSettings::default()))
        ));

        assert!(state.is_loading);
        assert_eq!(state.request_id, current_request_id);
        assert!(state.player_data.is_none());
    }

    #[test]
    fn cancel_search_ignores_later_result() {
        let mut state = AppState::new();
        let (request_id, _) = state.begin_search("test").unwrap();

        assert!(state.cancel_search());
        assert!(!state.is_loading);
        assert_eq!(state.loading_query, None);

        assert!(!state.finish_search(
            request_id,
            Ok(player_with_crosshair(CrosshairSettings::default()))
        ));
        assert!(state.player_data.is_none());
    }
}
