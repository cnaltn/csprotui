use crate::models::CrosshairSettings;

const DICTIONARY: &str = "ABCDEFGHJKLMNPRSTUVWXYZ23456789";
const DICTIONARY_LEN: u128 = 32;

pub fn encode_crosshair(settings: &CrosshairSettings) -> Option<String> {
    let crosshair = parse_crosshair_config(settings)?;

    let bytes = encode_crosshair_bytes(&crosshair);

    let share_code = bytes_to_share_code(&bytes);

    Some(share_code)
}

fn parse_crosshair_config(settings: &CrosshairSettings) -> Option<CsgoCrosshair> {
    Some(CsgoCrosshair {
        style: parse_f32(settings.style.as_deref()).round() as u8,
        length: parse_f32(settings.size.as_deref()),
        thickness: parse_f32(settings.thickness.as_deref()),
        gap: parse_f32(settings.gap.as_deref()),
        outline: parse_f32(settings.outline_thickness.as_deref()),
        color: parse_color(settings.color.as_deref()),
        red: parse_u8(settings.color_r.as_deref()),
        green: parse_u8(settings.color_g.as_deref()),
        blue: parse_u8(settings.color_b.as_deref()),
        alpha: parse_u8(settings.alpha.as_deref()),
        follow_recoil: is_true(settings.follow_recoil.as_deref()),
        outline_enabled: is_true(settings.draw_outline.as_deref()),
        center_dot_enabled: is_true(settings.dot.as_deref()),
        t_style_enabled: is_true(settings.t_style.as_deref()),
        deployed_weapon_gap_enabled: is_true(settings.deployed_weapon_gap.as_deref()),
        alpha_enabled: is_true(settings.use_alpha.as_deref()),
        split_distance: parse_f32(settings.split_distance.as_deref()),
        fixed_crosshair_gap: parse_f32(settings.fixed_gap.as_deref()),
        inner_split_alpha: parse_f32(settings.inner_split_alpha.as_deref()),
        outer_split_alpha: parse_f32(settings.outer_split_alpha.as_deref()),
        split_size_ratio: parse_f32(settings.split_size_ratio.as_deref()),
    })
}

fn encode_crosshair_bytes(c: &CsgoCrosshair) -> Vec<u8> {
    let mut bytes = vec![0u8; 18];

    bytes[0] = 0;
    bytes[1] = 1;
    bytes[2] = (c.gap * 10.0) as u8;
    bytes[3] = (c.outline * 2.0) as u8;
    bytes[4] = c.red;
    bytes[5] = c.green;
    bytes[6] = c.blue;
    bytes[7] = c.alpha;
    bytes[8] = ((c.split_distance as u8) & 0x7) | ((c.follow_recoil as u8) << 7);
    bytes[9] = (c.fixed_crosshair_gap * 10.0) as u8;
    bytes[10] = (c.color & 0x7)
        | ((c.outline_enabled as u8) << 3)
        | (((c.inner_split_alpha * 10.0) as u8) << 4);
    bytes[11] = ((c.outer_split_alpha * 10.0) as u8) | (((c.split_size_ratio * 10.0) as u8) << 4);
    bytes[12] = (c.thickness * 10.0) as u8;
    bytes[13] = (c.style << 1)
        | ((c.center_dot_enabled as u8) << 4)
        | ((c.deployed_weapon_gap_enabled as u8) << 5)
        | ((c.alpha_enabled as u8) << 6)
        | ((c.t_style_enabled as u8) << 7);
    bytes[14] = (c.length * 10.0) as u8;
    bytes[15] = 0;
    bytes[16] = 0;
    bytes[17] = 0;

    let sum: u32 = bytes[1..].iter().map(|&b| b as u32).sum();
    bytes[0] = (sum & 0xFF) as u8;

    bytes
}

fn bytes_to_share_code(bytes: &[u8]) -> String {
    let hex_str: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    let mut t = u128::from_str_radix(&hex_str, 16).unwrap_or(0);
    let mut result = String::new();

    for _ in 0..25 {
        let idx = (t % DICTIONARY_LEN) as usize;
        result.push(DICTIONARY.chars().nth(idx).unwrap_or('A'));
        t /= DICTIONARY_LEN;
    }

    format!(
        "CSGO-{}-{}-{}-{}-{}",
        &result[0..5],
        &result[5..10],
        &result[10..15],
        &result[15..20],
        &result[20..25]
    )
}

#[derive(Debug, Clone)]
struct CsgoCrosshair {
    style: u8,
    length: f32,
    thickness: f32,
    gap: f32,
    outline: f32,
    color: u8,
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
    follow_recoil: bool,
    outline_enabled: bool,
    center_dot_enabled: bool,
    t_style_enabled: bool,
    deployed_weapon_gap_enabled: bool,
    alpha_enabled: bool,
    split_distance: f32,
    fixed_crosshair_gap: f32,
    inner_split_alpha: f32,
    outer_split_alpha: f32,
    split_size_ratio: f32,
}

fn parse_f32(s: Option<&str>) -> f32 {
    s.and_then(|v| v.parse().ok()).unwrap_or(0.0)
}

fn parse_u8(s: Option<&str>) -> u8 {
    s.and_then(|v| v.parse().ok()).unwrap_or(0)
}

fn parse_color(s: Option<&str>) -> u8 {
    match s {
        Some("Red") => 1,
        Some("Yellow") => 2,
        Some("Green") => 0,
        Some("Cyan") => 3,
        Some("Blue") => 4,
        Some("Magenta") => 5,
        Some("White") => 6,
        Some("Custom") => 7,
        _ => 0,
    }
}

fn is_true(s: Option<&str>) -> bool {
    matches!(s, Some("Yes") | Some("1") | Some("true"))
}

#[cfg(test)]
mod tests {
    use super::encode_crosshair;
    use crate::models::CrosshairSettings;

    #[test]
    fn encode_crosshair_keeps_existing_share_code_format() {
        let settings = CrosshairSettings {
            import_code: None,
            command: None,
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

        assert_eq!(
            encode_crosshair(&settings).as_deref(),
            Some("CSGO-AAAAA-AAAAA-AAAAA-AAAAA-AAAAA")
        );
    }
}
