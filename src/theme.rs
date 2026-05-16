use opaline::{current, list_available_themes, load_by_name, set_theme, Theme, ThemeInfo};

pub struct ThemeManager {
    themes: Vec<ThemeInfo>,
    current_index: usize,
}

impl ThemeManager {
    pub fn new() -> Self {
        let themes = list_available_themes();
        let default_theme = Theme::default();
        set_theme(default_theme);
        Self {
            themes,
            current_index: 0,
        }
    }

    pub fn current_theme(&self) -> Theme {
        (*current()).clone()
    }

    pub fn current_theme_name(&self) -> String {
        let global = current();
        if let Some(pos) = self.themes.iter().position(|t| t.name == global.meta.name) {
            self.themes[pos].name.clone()
        } else {
            "default".to_string()
        }
    }

    pub fn filter_themes(&self, filter: &str) -> Vec<String> {
        let filter = filter.to_lowercase();
        self.themes
            .iter()
            .filter(|t| filter.is_empty() || t.name.to_lowercase().contains(&filter))
            .map(|t| t.name.clone())
            .collect()
    }

    pub fn select_theme(&mut self, name: &str) {
        if let Some(pos) = self.themes.iter().position(|t| t.name == name) {
            self.current_index = pos;
            if let Some(theme) = load_by_name(name) {
                set_theme(theme);
            }
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
