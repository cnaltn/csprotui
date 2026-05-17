use opaline::{current, list_available_themes, load_by_name, set_theme, Theme, ThemeInfo};
use std::fs;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    let base = if cfg!(windows) {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    } else {
        std::env::var("HOME")
            .map(|h| PathBuf::from(h).join(".config"))
            .unwrap_or_else(|_| PathBuf::from("."))
    };
    base.join("csprotui")
}

fn theme_file() -> PathBuf {
    config_dir().join("theme")
}

fn read_saved_theme() -> Option<String> {
    let path = theme_file();
    if path.exists() {
        let name = fs::read_to_string(&path).ok()?;
        let name = name.trim().to_string();
        if !name.is_empty() && load_by_name(&name).is_some() {
            return Some(name);
        }
    }
    None
}

fn save_theme(name: &str) {
    let dir = config_dir();
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(theme_file(), name);
}

pub struct ThemeManager {
    themes: Vec<ThemeInfo>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let themes = list_available_themes();

        // Load saved theme, fallback to Flexoki Dark
        let name = read_saved_theme()
            .unwrap_or_else(|| "flexoki-dark".to_string());

        let theme = load_by_name(&name).unwrap_or_else(|| {
            load_by_name("flexoki-dark").unwrap_or_else(Theme::default)
        });
        set_theme(theme);

        Self { themes }
    }

    pub fn current_theme(&self) -> Theme {
        (*current()).clone()
    }

    pub fn current_theme_name(&self) -> String {
        let global = current();
        global.meta.name.to_string()
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
        if let Some(theme) = load_by_name(name) {
            set_theme(theme);
            save_theme(name);
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
