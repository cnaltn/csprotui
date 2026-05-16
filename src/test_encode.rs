fn main() {
    let settings = crate::models::CrosshairSettings {
        import_code: None,
        style: Some("Classic Static".to_string()),
        follow_recoil: Some("No".to_string()),
        dot: Some("No".to_string()),
        size: Some("3".to_string()),
        thickness: Some("0.5".to_string()),
        gap: Some("0".to_string()),
        draw_outline: Some("No".to_string()),
        outline_thickness: Some("1".to_string()),
        color: Some("Green".to_string()),
        color_r: Some("50".to_string()),
        color_g: Some("250".to_string()),
        color_b: Some("50".to_string()),
        use_alpha: Some("Yes".to_string()),
        alpha: Some("200".to_string()),
        t_style: Some("No".to_string()),
        deployed_weapon_gap: Some("No".to_string()),
        split_distance: Some("7".to_string()),
        fixed_gap: Some("3".to_string()),
        inner_split_alpha: Some("1".to_string()),
        outer_split_alpha: Some("0.5".to_string()),
        split_size_ratio: Some("0.3".to_string()),
        sniper_width: Some("0".to_string()),
    };

    let result = crate::crosshair::encode_crosshair(&settings);
    println!("Result: {:?}", result);
}