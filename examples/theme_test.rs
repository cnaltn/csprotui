fn main() {
    let manager = prosettings_tui::theme::ThemeManager::new();

    println!("Initial theme: {}", manager.current_theme_name());
    let c = manager.current_theme().color("accent.primary");
    println!("  accent.primary: R={} G={} B={}", c.r, c.g, c.b);
    let c = manager.current_theme().color("text.primary");
    println!("  text.primary: R={} G={} B={}", c.r, c.g, c.b);
    let c = manager.current_theme().color("text.secondary");
    println!("  text.secondary: R={} G={} B={}", c.r, c.g, c.b);

    let themes = manager.filter_themes("");
    for (i, name) in themes.iter().enumerate().take(5) {
        println!("\nTheme: {}", name);
        let c = manager.current_theme().color("accent.primary");
        println!("  accent.primary: R={} G={} B={}", c.r, c.g, c.b);
        let c = manager.current_theme().color("text.primary");
        println!("  text.primary: R={} G={} B={}", c.r, c.g, c.b);
        let _ = i;
    }
}
