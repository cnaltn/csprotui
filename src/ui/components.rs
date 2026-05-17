use ratatui::{
    prelude::*,
    widgets::{
        Block, Borders, Cell, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, Tabs,
    },
    Frame,
};

use crate::models::{
    CrosshairSettings, HudSettings, MouseSettings, PlayerData, RadarSettings, Tab, VideoSettings,
    ViewmodelSettings,
};
use crate::ui::App;
use opaline::names::tokens;
use opaline::widgets::ThemeSelector;
use opaline::Theme;
use tui_banner::{Banner, ColorMode, Fill};

fn c(theme: &Theme, token: &str) -> Color {
    theme.color(token).into()
}

fn style_muted(theme: &Theme) -> Style {
    Style::default().fg(c(theme, tokens::TEXT_MUTED))
}

fn style_key(theme: &Theme) -> Style {
    Style::default()
        .fg(c(theme, tokens::ACCENT_SECONDARY))
        .bold()
}

fn style_value(theme: &Theme) -> Style {
    Style::default().fg(c(theme, tokens::TEXT_PRIMARY))
}

fn dim_if(theme: &Theme, style: Style, dimmed: bool) -> Style {
    if dimmed {
        style
            .fg(c(theme, tokens::TEXT_MUTED))
            .add_modifier(Modifier::DIM)
    } else {
        style
    }
}

fn panel_border(theme: &Theme) -> Style {
    Style::default().fg(c(theme, tokens::BORDER_UNFOCUSED))
}

fn panel_title(theme: &Theme) -> Style {
    Style::default()
        .fg(c(theme, tokens::TEXT_PRIMARY))
        .add_modifier(Modifier::BOLD)
}

fn block_with_bg_dimmed(theme: &Theme, dimmed: bool) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(dim_if(theme, panel_border(theme), dimmed))
        .style(Style::default().bg(c(theme, tokens::BG_BASE)))
}

fn block_with_title<'a>(theme: &Theme, title: &'a str) -> Block<'a> {
    block_with_title_dimmed(theme, title, false)
}

fn block_with_title_dimmed<'a>(theme: &Theme, title: &'a str, dimmed: bool) -> Block<'a> {
    block_with_bg_dimmed(theme, dimmed)
        .title(title.to_string())
        .title_style(dim_if(theme, panel_title(theme), dimmed))
}

fn footer_bg(theme: &Theme) -> Style {
    Style::default()
        .bg(c(theme, tokens::BG_PANEL))
        .fg(c(theme, tokens::TEXT_MUTED))
}

fn inner_rect(area: Rect) -> Rect {
    Rect::new(
        area.x.saturating_add(1),
        area.y.saturating_add(1),
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    )
}

fn visible_items<'a>(items: &[(&'static str, Option<&'a str>)]) -> Vec<(&'static str, &'a str)> {
    items
        .iter()
        .filter_map(|(key, value)| {
            let value = (*value)?;
            let trimmed = value.trim();
            (!trimmed.is_empty() && trimmed != "-").then_some((*key, value))
        })
        .collect()
}

fn truncate_to_width(value: &str, width: u16) -> String {
    let width = width as usize;
    if value.chars().count() <= width {
        return value.to_string();
    }
    if width <= 3 {
        return ".".repeat(width);
    }
    let mut clipped: String = value.chars().take(width - 3).collect();
    clipped.push_str("...");
    clipped
}

#[derive(Clone, Copy)]
struct SettingsTableView {
    tab: Tab,
    scroll_offset: usize,
    dimmed: bool,
}

impl App {
    pub fn render(&self, f: &mut Frame) {
        let mut state = self.state.lock().unwrap();
        let theme = state.theme_manager.current_theme().clone();
        let animation_elapsed_ms = state.animation_started_at.elapsed().as_millis();
        let toast_visible = state.toast_visible;
        let toast_msg = state.toast_msg.clone();

        if state.on_landing {
            self.render_landing(f, &state, &theme, animation_elapsed_ms);

            if toast_visible {
                let area = f.area();
                let toast_width = (toast_msg.len() as u16 + 4).min(40);
                let toast_area = Rect {
                    x: area.width.saturating_sub(toast_width + 2),
                    y: 0,
                    width: toast_width,
                    height: 3,
                };
                let toast = Paragraph::new(toast_msg)
                    .style(
                        Style::default()
                            .fg(c(&theme, tokens::BG_BASE))
                            .bg(c(&theme, tokens::SUCCESS)),
                    )
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(c(&theme, tokens::SUCCESS)))
                            .style(Style::default().bg(c(&theme, tokens::BG_BASE))),
                    );
                f.render_widget(Clear, toast_area);
                f.render_widget(toast, toast_area);
            }

            if let Some(ref mut selector) = state.theme_selector {
                let popup = centered_rect(60, 70, f.area());
                let popup_bg = Block::default().style(Style::default().bg(c(&theme, tokens::BG_BASE)));
                f.render_widget(Clear, popup);
                f.render_widget(popup_bg, popup);
                f.render_stateful_widget(ThemeSelector::new(), popup, selector);
            }
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        self.render_header(f, chunks[0], &state, &theme, animation_elapsed_ms);
        self.render_content(f, chunks[1], &mut state, &theme);
        let theme_name = state.theme_manager.current_theme_name();
        self.render_footer(
            f,
            chunks[2],
            &theme_name,
            &theme,
            state.player_data.is_some(),
        );

        if toast_visible {
            let area = f.area();
            let toast_width = (toast_msg.len() as u16 + 4).min(40);
            let toast_area = Rect {
                x: area.width.saturating_sub(toast_width + 2),
                y: 0,
                width: toast_width,
                height: 3,
            };
            let toast = Paragraph::new(toast_msg)
                .style(
                    Style::default()
                        .fg(c(&theme, tokens::BG_BASE))
                        .bg(c(&theme, tokens::SUCCESS)),
                )
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(c(&theme, tokens::SUCCESS)))
                        .style(Style::default().bg(c(&theme, tokens::BG_BASE))),
                );
            f.render_widget(Clear, toast_area);
            f.render_widget(toast, toast_area);
        }

        if let Some(ref mut selector) = state.theme_selector {
            let popup = centered_rect(60, 70, f.area());
            let popup_bg = Block::default().style(Style::default().bg(c(&theme, tokens::BG_BASE)));
            f.render_widget(Clear, popup);
            f.render_widget(popup_bg, popup);
            f.render_stateful_widget(ThemeSelector::new(), popup, selector);
        }
    }

    fn render_landing(
        &self,
        f: &mut Frame,
        state: &crate::ui::app::AppState,
        theme: &Theme,
        animation_elapsed_ms: u128,
    ) {
        let area = f.area();

        f.render_widget(
            Block::default().style(Style::default().bg(c(theme, tokens::BG_BASE))),
            area,
        );

        // Banner text with gradient
        let banner_lines = Banner::new("CSPROTUI")
            .ok()
            .map(|b| b.color_mode(ColorMode::NoColor).fill(Fill::Keep).render())
            .unwrap_or_default();
        let banner_count = banner_lines.lines().count() as u16;

        let accent = c(theme, tokens::ACCENT_PRIMARY);
        let (ar, ag, ab) = match accent {
            Color::Rgb(r, g, b) => (r, g, b),
            _ => (128, 128, 128),
        };
        let total_rows = banner_lines.lines().count().max(1) as f32;
        let mut lines = Vec::new();
        for (row, line) in banner_lines.lines().enumerate() {
            let t = if total_rows > 1.0 {
                row as f32 / (total_rows - 1.0)
            } else {
                0.0
            };
            let spans: Vec<Span> = line
                .chars()
                .map(|c| {
                    if c == ' ' {
                        Span::styled(" ", Style::default())
                    } else {
                        let r = ((ar as f32) * (1.0 - t) + 255.0 * t) as u8;
                        let g = ((ag as f32) * (1.0 - t) + 255.0 * t) as u8;
                        let b = ((ab as f32) * (1.0 - t) + 255.0 * t) as u8;
                        Span::styled(c.to_string(), Style::default().fg(Color::Rgb(r, g, b)))
                    }
                })
                .collect();
            lines.push(Line::from(spans));
        }
        let banner_p = Paragraph::new(lines).alignment(Alignment::Center);

        let desc = Paragraph::new("TUI for Counter-Strike pro player settings")
            .style(style_muted(theme))
            .alignment(Alignment::Center);

        // Input line with accent cursor
        let cursor_visible = (animation_elapsed_ms / 500).is_multiple_of(2);
        let query = &state.search_query;
        let input_spans = if query.is_empty() {
            vec![Span::styled(
                "Search player (xantares, s1mple, ZywOo)...",
                style_muted(theme).italic(),
            )]
        } else {
            let mut s = vec![Span::styled(query.as_str(), style_muted(theme))];
            if cursor_visible {
                s.push(Span::styled(
                    "█",
                    Style::default().fg(c(theme, tokens::ACCENT_PRIMARY)),
                ));
            } else {
                s.push(Span::styled(" ", Style::default()));
            }
            s
        };
        let input_p = Paragraph::new(Line::from(input_spans))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(c(theme, tokens::ACCENT_PRIMARY)))
                    .title(" Search ")
                    .title_style(
                        Style::default()
                            .fg(c(theme, tokens::ACCENT_PRIMARY))
                            .add_modifier(Modifier::BOLD),
                    )
                    .style(Style::default().bg(c(theme, tokens::BG_BASE))),
            );

        let content_h = banner_count + 1 + 1 + 3; // banner + gap + desc + input(box)
        let top_pad = if content_h < area.height {
            (area.height - content_h) / 2
        } else {
            0
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(top_pad),
                Constraint::Length(banner_count),
                Constraint::Length(1),  // gap
                Constraint::Length(1),  // desc
                Constraint::Length(3),  // input box
                Constraint::Min(0),
            ])
            .split(area);

        f.render_widget(banner_p, layout[1]);
        f.render_widget(desc, layout[3]);

        // Centered narrow input area
        let input_w = layout[4].width.min(50);
        let input_x = layout[4].x + (layout[4].width.saturating_sub(input_w)) / 2;
        let input_area = Rect::new(input_x, layout[4].y, input_w, layout[4].height);
        f.render_widget(input_p, input_area);
    }

    fn render_header(
        &self,
        f: &mut Frame,
        area: Rect,
        state: &crate::ui::app::AppState,
        theme: &Theme,
        animation_elapsed_ms: u128,
    ) {
        let accent = c(theme, tokens::ACCENT_PRIMARY);
        let border_color = if state.is_loading { accent } else { c(theme, tokens::BORDER_UNFOCUSED) };
        let title_color = if state.is_loading { accent } else { c(theme, tokens::TEXT_PRIMARY) };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(" Search ")
            .title_style(
                Style::default()
                    .fg(title_color)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(c(theme, tokens::BG_BASE)));
        f.render_widget(block, area);

        let inner = Rect::new(
            area.x.saturating_add(1),
            area.y.saturating_add(1),
            area.width.saturating_sub(2),
            1,
        );

        if state.is_loading {
            let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let s = spinner[((animation_elapsed_ms / 100) as usize) % spinner.len()];
            let query = state.loading_query.as_deref().or_else(|| {
                (!state.search_query.trim().is_empty()).then_some(state.search_query.trim())
            });
            let search_text = if let Some(query) = query {
                format!("{s} Searching {query}...")
            } else {
                format!("{s} Searching player settings...")
            };
            let paragraph = Paragraph::new(search_text)
                .style(
                    Style::default()
                        .fg(accent)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default());
            f.render_widget(paragraph, inner);
        } else if !state.search_query.is_empty() {
            let cursor_visible = (animation_elapsed_ms / 500).is_multiple_of(2);
            let mut spans = vec![
                Span::styled("> ", Style::default().fg(accent)),
                Span::styled(&state.search_query, style_value(theme)),
            ];
            if cursor_visible {
                spans.push(Span::styled(
                    "█",
                    Style::default().fg(accent),
                ));
            } else {
                spans.push(Span::styled(" ", Style::default()));
            }
            let paragraph = Paragraph::new(Line::from(spans)).block(Block::default());
            f.render_widget(paragraph, inner);
        } else {
            let paragraph = Paragraph::new("> Search player (xantares, s1mple, ZywOo)...")
                .style(style_muted(theme).italic())
                .block(Block::default());
            f.render_widget(paragraph, inner);
        }
    }

    fn render_content(
        &self,
        f: &mut Frame,
        area: Rect,
        state: &mut crate::ui::app::AppState,
        theme: &Theme,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        state.settings_rect = Some(chunks[2]);

        if let Some(player) = state.player_data.as_ref() {
            let dimmed = state.is_loading;
            self.render_player_info(f, chunks[0], player, theme, dimmed);
            self.render_tabs(f, chunks[1], state.current_tab, theme, dimmed);
            self.render_settings_table(
                f,
                chunks[2],
                player,
                theme,
                SettingsTableView {
                    tab: state.current_tab,
                    scroll_offset: state.scroll_offset,
                    dimmed,
                },
            );

            let import_code = player
                .data
                .crosshair
                .import_code
                .as_deref()
                .filter(|code| !code.trim().is_empty());
            self.render_import_code(f, chunks[3], import_code, state.copied, theme, dimmed);
        } else {
            if let Some(err) = state.error_message.as_ref() {
                self.render_error_block(f, chunks[0], err, theme);
            } else {
                let paragraph = Paragraph::new("Enter a player name and press Enter")
                    .style(style_muted(theme))
                    .block(block_with_title(theme, " Player Info "))
                    .alignment(Alignment::Center);
                f.render_widget(paragraph, chunks[0]);
            }

            let tabs = self.create_tabs_widget(state.current_tab, theme, false);
            f.render_widget(tabs, chunks[1]);

            if state.error_message.is_some() {
                self.render_no_result_block(f, chunks[2], " Settings ", theme);
            } else {
                let placeholder = Paragraph::new("No player loaded")
                    .style(style_muted(theme))
                    .alignment(Alignment::Center)
                    .block(block_with_title(theme, " Settings "));
                f.render_widget(placeholder, chunks[2]);
            }

            if state.error_message.is_some() {
                self.render_no_result_block(f, chunks[3], " Import Code CTRL + Y to Copy ", theme);
            } else {
                let import_para = Paragraph::new("No import code")
                    .style(style_muted(theme))
                    .alignment(Alignment::Center)
                    .block(block_with_title(theme, " Import Code CTRL + Y to Copy "));
                f.render_widget(import_para, chunks[3]);
            }
        }
    }

    fn render_no_result_block(&self, f: &mut Frame, area: Rect, title: &str, theme: &Theme) {
        let paragraph = Paragraph::new("No result")
            .style(Style::default().fg(c(theme, tokens::WARNING)))
            .alignment(Alignment::Center)
            .block(
                block_with_title(theme, title)
                    .border_style(Style::default().fg(c(theme, tokens::WARNING))),
            );
        f.render_widget(paragraph, area);
    }

    fn render_player_info(
        &self,
        f: &mut Frame,
        area: Rect,
        player: &PlayerData,
        theme: &Theme,
        dimmed: bool,
    ) {
        let block = block_with_title_dimmed(theme, " Player Info ", dimmed);
        f.render_widget(block, area);

        let inner = Rect::new(
            area.x.saturating_add(1),
            area.y.saturating_add(1),
            area.width.saturating_sub(2),
            1,
        );
        let paragraph = Paragraph::new(Line::from(vec![Span::styled(
            player.player.as_str(),
            dim_if(
                theme,
                Style::default().fg(c(theme, tokens::ACCENT_PRIMARY)).bold(),
                dimmed,
            ),
        )]))
        .alignment(Alignment::Left);
        f.render_widget(paragraph, inner);
    }

    fn render_tabs(
        &self,
        f: &mut Frame,
        area: Rect,
        current_tab: Tab,
        theme: &Theme,
        dimmed: bool,
    ) {
        let tabs = self.create_tabs_widget(current_tab, theme, dimmed);
        f.render_widget(tabs, area);
    }

    fn create_tabs_widget(&self, current_tab: Tab, theme: &Theme, dimmed: bool) -> Tabs<'_> {
        let titles: Vec<Line> = Tab::ALL
            .iter()
            .map(|tab| Line::from(tab.short_name()))
            .collect();

        Tabs::new(titles)
            .block(block_with_bg_dimmed(theme, dimmed))
            .select(current_tab.index())
            .style(dim_if(theme, style_muted(theme), dimmed))
            .highlight_style(dim_if(
                theme,
                Style::default()
                    .fg(c(theme, tokens::TEXT_PRIMARY))
                    .bg(c(theme, tokens::BG_CODE))
                    .add_modifier(Modifier::BOLD),
                dimmed,
            ))
            .divider(" ")
    }

    fn render_settings_table(
        &self,
        f: &mut Frame,
        area: Rect,
        player: &PlayerData,
        theme: &Theme,
        view: SettingsTableView,
    ) {
        let title = format!(" {} ", view.tab);
        let block = block_with_title_dimmed(theme, &title, view.dimmed);
        f.render_widget(block, area);

        let inner = inner_rect(area);
        match view.tab {
            Tab::Mouse => self.render_mouse_table(
                f,
                inner,
                &player.data.mouse,
                theme,
                view.scroll_offset,
                view.dimmed,
            ),
            Tab::Crosshair => self.render_crosshair_table(
                f,
                inner,
                &player.data.crosshair,
                theme,
                view.scroll_offset,
                view.dimmed,
            ),
            Tab::Viewmodel => self.render_viewmodel_table(
                f,
                inner,
                &player.data.viewmodel,
                theme,
                view.scroll_offset,
                view.dimmed,
            ),
            Tab::Video => self.render_video_table(
                f,
                inner,
                &player.data.video,
                theme,
                view.scroll_offset,
                view.dimmed,
            ),
            Tab::Radar => self.render_radar_table(
                f,
                inner,
                &player.data.radar,
                theme,
                view.scroll_offset,
                view.dimmed,
            ),
            Tab::Hud => self.render_hud_table(
                f,
                inner,
                &player.data.hud,
                theme,
                view.scroll_offset,
                view.dimmed,
            ),
            Tab::LaunchOptions => self.render_launch_options_table(
                f,
                inner,
                &player.data.launch_options,
                theme,
                view.scroll_offset,
                view.dimmed,
            ),
            Tab::Gear => {
                let items: Vec<(String, String)> = player
                    .data
                    .gear
                    .iter()
                    .map(|g| (g.name.clone(), g.category.clone()))
                    .collect();
                self.render_items_table(
                    f,
                    inner,
                    &items,
                    theme,
                    view.scroll_offset,
                    view.dimmed,
                );
            }
        }
    }

    fn render_mouse_table(
        &self,
        f: &mut Frame,
        area: Rect,
        mouse: &MouseSettings,
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let items = visible_items(&[
            ("DPI", mouse.dpi.as_deref()),
            ("Sensitivity", mouse.sensitivity.as_deref()),
            ("eDPI", mouse.edpi.as_deref()),
            (
                "Zoom Sensitivity",
                mouse.zoom_sensitivity_ratio_mouse.as_deref(),
            ),
            ("Hz", mouse.hz.as_deref()),
            ("Windows Sensitivity", mouse.windows_sensitivity.as_deref()),
        ]);
        self.render_key_value_table(f, area, &items, theme, scroll_offset, dimmed);
    }

    fn render_crosshair_table(
        &self,
        f: &mut Frame,
        area: Rect,
        crosshair: &CrosshairSettings,
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let items = visible_items(&[
            ("Style", crosshair.style.as_deref()),
            ("Follow Recoil", crosshair.follow_recoil.as_deref()),
            ("Dot", crosshair.dot.as_deref()),
            ("Size", crosshair.size.as_deref()),
            ("Thickness", crosshair.thickness.as_deref()),
            ("Gap", crosshair.gap.as_deref()),
            ("Outline", crosshair.draw_outline.as_deref()),
            ("Outline Thickness", crosshair.outline_thickness.as_deref()),
            ("Color", crosshair.color.as_deref()),
            ("R", crosshair.color_r.as_deref()),
            ("G", crosshair.color_g.as_deref()),
            ("B", crosshair.color_b.as_deref()),
            ("Alpha", crosshair.alpha.as_deref()),
            ("T Style", crosshair.t_style.as_deref()),
            (
                "Deployed Weapon Gap",
                crosshair.deployed_weapon_gap.as_deref(),
            ),
            ("Split Distance", crosshair.split_distance.as_deref()),
            ("Fixed Gap", crosshair.fixed_gap.as_deref()),
            ("Inner Split Alpha", crosshair.inner_split_alpha.as_deref()),
            ("Outer Split Alpha", crosshair.outer_split_alpha.as_deref()),
            ("Split Size Ratio", crosshair.split_size_ratio.as_deref()),
            ("Sniper Width", crosshair.sniper_width.as_deref()),
        ]);

        self.render_key_value_table(f, area, &items, theme, scroll_offset, dimmed);
    }

    fn render_video_table(
        &self,
        f: &mut Frame,
        area: Rect,
        video: &VideoSettings,
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let items = visible_items(&[
            ("Resolution", video.resolution.as_deref()),
            ("Aspect Ratio", video.aspect_ratio.as_deref()),
            ("Scaling Mode", video.scaling_mode.as_deref()),
            ("Brightness", video.brightness.as_deref()),
            ("Display Mode", video.display_mode.as_deref()),
            ("Boost Contrast", video.boost_contrast.as_deref()),
            ("VSync", video.vsync.as_deref()),
            ("NVIDIA Reflex", video.reflex_low_latency.as_deref()),
            ("G-Sync", video.gsync.as_deref()),
            ("Max FPS", video.max_fps.as_deref()),
            ("Anti-Aliasing", video.anti_aliasing.as_deref()),
            ("Shadow Quality", video.shadow_quality.as_deref()),
            ("Dynamic Shadows", video.dynamic_shadows.as_deref()),
            ("Texture Detail", video.texture_detail.as_deref()),
            ("Filtering Mode", video.filtering_mode.as_deref()),
            ("Shader Detail", video.shader_detail.as_deref()),
            ("Particle Detail", video.particle_detail.as_deref()),
            ("Ambient Occlusion", video.ambient_occlusion.as_deref()),
            ("HDR", video.hdr.as_deref()),
            ("FSR", video.fsr.as_deref()),
        ]);
        self.render_key_value_table(f, area, &items, theme, scroll_offset, dimmed);
    }

    fn render_radar_table(
        &self,
        f: &mut Frame,
        area: Rect,
        radar: &RadarSettings,
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let items = visible_items(&[
            ("Always Centered", radar.always_centered.as_deref()),
            ("Rotate", radar.rotate.as_deref()),
            (
                "Square with Scoreboard",
                radar.square_with_scoreboard.as_deref(),
            ),
            ("HUD Scale", radar.hud_scale.as_deref()),
            ("Scale", radar.scale.as_deref()),
        ]);
        self.render_key_value_table(f, area, &items, theme, scroll_offset, dimmed);
    }

    fn render_hud_table(
        &self,
        f: &mut Frame,
        area: Rect,
        hud: &HudSettings,
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let items = visible_items(&[
            ("HUD Scaling", hud.scaling.as_deref()),
            ("HUD Color", hud.color.as_deref()),
        ]);
        self.render_key_value_table(f, area, &items, theme, scroll_offset, dimmed);
    }

    fn render_viewmodel_table(
        &self,
        f: &mut Frame,
        area: Rect,
        viewmodel: &ViewmodelSettings,
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let items = visible_items(&[
            ("FOV", viewmodel.fov.as_deref()),
            ("Offset X", viewmodel.offset_x.as_deref()),
            ("Offset Y", viewmodel.offset_y.as_deref()),
            ("Offset Z", viewmodel.offset_z.as_deref()),
            ("Preset", viewmodel.presetpos.as_deref()),
            ("Use New Bob", viewmodel.use_new_bob.as_deref()),
        ]);
        self.render_key_value_table(f, area, &items, theme, scroll_offset, dimmed);
    }

    fn render_launch_options_table(
        &self,
        f: &mut Frame,
        area: Rect,
        launch_options: &Option<String>,
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let items = visible_items(&[("Launch Options", launch_options.as_deref())]);
        self.render_key_value_table(f, area, &items, theme, scroll_offset, dimmed);
    }

    fn render_items_table(
        &self,
        f: &mut Frame,
        area: Rect,
        items: &[(String, String)],
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let bg = c(theme, tokens::BG_BASE);
        let accent = c(theme, tokens::ACCENT_PRIMARY);

        if items.is_empty() {
            let placeholder = Paragraph::new("No items available")
                .style(dim_if(theme, style_muted(theme), dimmed))
                .alignment(Alignment::Center);
            f.render_widget(placeholder, area);
            return;
        }

        let header = Row::new(vec![Cell::from("Item"), Cell::from("Category")]).style(dim_if(
            theme,
            Style::default().fg(accent).bg(bg).bold(),
            dimmed,
        ));

        let all_rows: Vec<Row> = items
            .iter()
            .enumerate()
            .map(|(idx, (name, category))| {
                let row_bg = if idx % 2 == 0 {
                    bg
                } else {
                    c(theme, tokens::BG_PANEL)
                };
                let key_style = dim_if(theme, style_key(theme).bg(row_bg), dimmed);
                let val_style = dim_if(theme, style_value(theme).bg(row_bg), dimmed);
                Row::new(vec![
                    Cell::from(name.clone()).style(key_style),
                    Cell::from(category.clone()).style(val_style),
                ])
            })
            .collect();

        let total_items = all_rows.len();
        let header_height = 1u16;
        let content_height = area.height.saturating_sub(header_height);
        let visible_rows = content_height as usize;
        let max_offset = total_items.saturating_sub(visible_rows.max(1));
        let offset = scroll_offset.min(max_offset);

        let needs_scroll = total_items > visible_rows;
        let scrollbar_width: u16 = if needs_scroll { 2 } else { 0 };
        let table_width = area.width.saturating_sub(scrollbar_width);

        let visible_slice: Vec<Row> = all_rows.into_iter().skip(offset).take(visible_rows).collect();

        let table = Table::new(
            visible_slice,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ],
        )
        .header(header)
        .column_spacing(1)
        .style(Style::default().bg(bg));

        f.render_widget(table, Rect::new(area.x, area.y, table_width, area.height));

        if needs_scroll {
            let sb_area = Rect::new(area.x + table_width, area.y + 1, 2, content_height);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_style(dim_if(
                    theme,
                    Style::default().fg(c(theme, tokens::ACCENT_SECONDARY)),
                    dimmed,
                ))
                .track_style(dim_if(
                    theme,
                    Style::default().fg(c(theme, tokens::BORDER_UNFOCUSED)),
                    dimmed,
                ));
            let mut sb_state = ScrollbarState::new(total_items)
                .position(offset)
                .viewport_content_length(visible_rows);
            f.render_stateful_widget(scrollbar, sb_area, &mut sb_state);
        }
    }

    fn render_key_value_table(
        &self,
        f: &mut Frame,
        area: Rect,
        items: &[(&str, &str)],
        theme: &Theme,
        scroll_offset: usize,
        dimmed: bool,
    ) {
        let bg = c(theme, tokens::BG_BASE);
        let accent = c(theme, tokens::ACCENT_PRIMARY);

        let header = Row::new(vec![Cell::from("Setting"), Cell::from("Value")]).style(dim_if(
            theme,
            Style::default().fg(accent).bg(bg).bold(),
            dimmed,
        ));

        let all_rows: Vec<Row> = items
            .iter()
            .enumerate()
            .map(|(idx, (key, value))| {
                let row_bg = if idx % 2 == 0 {
                    bg
                } else {
                    c(theme, tokens::BG_PANEL)
                };
                let key_style = dim_if(theme, style_key(theme).bg(row_bg), dimmed);
                let val_style = dim_if(theme, style_value(theme).bg(row_bg), dimmed);
                Row::new(vec![
                    Cell::from(*key).style(key_style),
                    Cell::from(*value).style(val_style),
                ])
            })
            .collect();

        if all_rows.is_empty() {
            let placeholder = Paragraph::new("No settings available for this tab")
                .style(dim_if(theme, style_muted(theme), dimmed))
                .alignment(Alignment::Center);
            f.render_widget(placeholder, area);
            return;
        }

        let total_items = all_rows.len();
        let header_height = 1u16;
        let content_height = area.height.saturating_sub(header_height);
        let visible_rows = content_height as usize;
        let max_offset = total_items.saturating_sub(visible_rows.max(1));
        let offset = scroll_offset.min(max_offset);

        let needs_scroll = total_items > visible_rows;
        let scrollbar_width: u16 = if needs_scroll { 2 } else { 0 };
        let table_width = area.width.saturating_sub(scrollbar_width);

        let visible_slice: Vec<Row> = all_rows
            .into_iter()
            .skip(offset)
            .take(visible_rows)
            .collect();

        let table = Table::new(
            visible_slice,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ],
        )
        .header(header)
        .column_spacing(1)
        .style(Style::default().bg(bg));

        f.render_widget(table, Rect::new(area.x, area.y, table_width, area.height));

        if needs_scroll {
            let sb_area = Rect::new(area.x + table_width, area.y + 1, 2, content_height);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_style(dim_if(
                    theme,
                    Style::default().fg(c(theme, tokens::ACCENT_SECONDARY)),
                    dimmed,
                ))
                .track_style(dim_if(
                    theme,
                    Style::default().fg(c(theme, tokens::BORDER_UNFOCUSED)),
                    dimmed,
                ));
            let mut sb_state = ScrollbarState::new(total_items)
                .position(offset)
                .viewport_content_length(visible_rows);
            f.render_stateful_widget(scrollbar, sb_area, &mut sb_state);
        }
    }

    fn render_import_code(
        &self,
        f: &mut Frame,
        area: Rect,
        code: Option<&str>,
        copied: bool,
        theme: &Theme,
        dimmed: bool,
    ) {
        let has_code = code.is_some();
        let token = if copied {
            tokens::SUCCESS
        } else if has_code {
            tokens::WARNING
        } else {
            tokens::TEXT_MUTED
        };
        let border_token = if copied {
            tokens::SUCCESS
        } else if has_code {
            tokens::WARNING
        } else {
            tokens::BORDER_UNFOCUSED
        };
        let title = " Import Code CTRL + Y to Copy ";
        let title_style = if copied || has_code {
            Style::default().fg(c(theme, token)).bold()
        } else {
            Style::default().fg(c(theme, token))
        };
        let block = block_with_bg_dimmed(theme, dimmed)
            .title(title)
            .title_style(dim_if(theme, title_style, dimmed))
            .border_style(dim_if(
                theme,
                Style::default().fg(c(theme, border_token)),
                dimmed,
            ));
        f.render_widget(block, area);

        let inner = Rect::new(
            area.x.saturating_add(1),
            area.y.saturating_add(1),
            area.width.saturating_sub(2),
            1,
        );
        let text = code.unwrap_or("No import code");
        let paragraph = Paragraph::new(truncate_to_width(text, inner.width))
            .style(dim_if(theme, Style::default().fg(c(theme, token)), dimmed))
            .alignment(Alignment::Center);
        f.render_widget(paragraph, inner);
    }

    fn render_footer(
        &self,
        f: &mut Frame,
        area: Rect,
        theme_name: &str,
        theme: &Theme,
        has_player: bool,
    ) {
        let copy_key = if has_player { " | CTRL+Y Copy" } else { "" };
        let text = if area.width < 72 {
            format!("Tab switch{copy_key} | Ctrl+T theme | Esc/q")
        } else {
            format!(" Tab/Shift+Tab: Tabs{copy_key} | Ctrl+T: Themes ({theme_name}) | Esc: Back | q: Quit ")
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(panel_border(theme))
            .style(footer_bg(theme));
        f.render_widget(block, area);

        let inner = Rect::new(
            area.x.saturating_add(1),
            area.y.saturating_add(1),
            area.width.saturating_sub(2),
            1,
        );
        let paragraph = Paragraph::new(truncate_to_width(&text, inner.width))
            .style(style_muted(theme))
            .alignment(Alignment::Center);
        f.render_widget(paragraph, inner);
    }

    fn render_error_block(&self, f: &mut Frame, area: Rect, err: &str, theme: &Theme) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(c(theme, tokens::WARNING)))
            .title(" Error ")
            .title_style(Style::default().fg(c(theme, tokens::WARNING)).bold())
            .style(Style::default().bg(c(theme, tokens::BG_BASE)));
        f.render_widget(block, area);

        let inner = inner_rect(area);
        let paragraph = Paragraph::new(err.to_string())
            .style(
                Style::default()
                    .fg(c(theme, tokens::WARNING))
                    .bg(c(theme, tokens::BG_BASE)),
            )
            .block(Block::default())
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(paragraph, inner);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::{truncate_to_width, visible_items};

    #[test]
    fn visible_items_filters_missing_empty_and_dash_values() {
        let items = visible_items(&[
            ("DPI", Some("400")),
            ("Empty", Some("")),
            ("Whitespace", Some("   ")),
            ("Dash", Some("-")),
            ("Missing", None),
        ]);

        assert_eq!(items, vec![("DPI", "400")]);
    }

    #[test]
    fn truncate_to_width_keeps_import_codes_inside_the_cell() {
        assert_eq!(truncate_to_width("CSGO-ABCDE", 20), "CSGO-ABCDE");
        assert_eq!(truncate_to_width("CSGO-ABCDE-FGHIJ", 10), "CSGO-AB...");
        assert_eq!(truncate_to_width("CSGO", 3), "...");
    }

    #[test]
    fn truncate_to_width_keeps_long_commands_inside_the_cell() {
        let command = "cl_crosshaircolor 5; cl_crosshairalpha 255; cl_crosshairgap -3";

        assert_eq!(truncate_to_width(command, 24), "cl_crosshaircolor 5; ...");
    }
}
