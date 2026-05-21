use crate::models::{PlayerData, PlayerSettings};
use ::scraper::{Html, Selector};
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

pub const TOTALCSGO_FETCH_TIMEOUT: Duration = Duration::from_secs(4);

fn scraper_dir() -> PathBuf {
    env::var("CSPROTUI_SCRAPER_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("scraper"))
}

fn scraper_path() -> PathBuf {
    scraper_dir().join("index.js")
}

fn base_url() -> Result<String, ScraperError> {
    // Runtime env var takes priority, then compile-time embed
    if let Ok(url) = env::var("CSPROTUI_BASE_URL") {
        return Ok(url);
    }
    if let Some(url) = option_env!("CSPROTUI_BASE_URL") {
        return Ok(url.to_string());
    }
    Err(ScraperError::ConfigMissing {
        key: "CSPROTUI_BASE_URL".to_string(),
    })
}

#[derive(Debug, thiserror::Error)]
pub enum ScraperError {
    #[error("Node.js is not installed or not in PATH.")]
    NodeNotFound,

    #[error("Failed to run scraper at {path}: {source}")]
    Command {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Player '{slug}' not found on prosettings.net.")]
    PlayerNotFound { slug: String },

    #[error("Access blocked by prosettings.net. Try again in a few seconds.")]
    AccessBlocked,

    #[error("Connection timed out. Check your internet connection.")]
    NetworkTimeout,

    #[error("Cannot reach prosettings.net. Check your internet connection.")]
    NetworkError,

    #[error("prosettings.net server error. Try again later.")]
    ServerError,

    #[error("Scraper failed{status}: {stderr}")]
    Exit { status: String, stderr: String },

    #[error("Failed to parse scraper JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Failed to fetch TotalCSGO crosshair: {0}")]
    TotalCsgoFetch(#[from] reqwest::Error),

    #[error("Missing config: {key}")]
    ConfigMissing { key: String },
}

impl ScraperError {
    /// Returns a user-friendly message suitable for UI display.
    pub fn user_message(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TotalCsgoCrosshair {
    pub code: String,
    pub command: String,
}

pub fn run_node_scraper(slug: &str) -> Result<String, ScraperError> {
    let path = scraper_path();
    let path_str = path.to_string_lossy().to_string();
    let output = Command::new("node")
        .arg(&path)
        .arg(slug)
        .env("CSPROTUI_BASE_URL", base_url()?)
        .output()
        .map_err(|source| {
            if source.kind() == std::io::ErrorKind::NotFound {
                ScraperError::NodeNotFound
            } else {
                ScraperError::Command {
                    path: path_str.clone(),
                    source,
                }
            }
        })?;

    if !output.status.success() {
        let status = output
            .status
            .code()
            .map(|code| format!(" (exit code {code})"))
            .unwrap_or_else(|| " (killed by signal)".to_string());
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        // Try to parse structured error JSON from the Node scraper
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&stderr) {
            if let Some(error_code) = value.get("error").and_then(|v| v.as_str()) {
                let message = value
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&stderr)
                    .to_string();
                match error_code {
                    "PLAYER_NOT_FOUND" => {
                        return Err(ScraperError::PlayerNotFound {
                            slug: slug.to_string(),
                        });
                    }
                    "ACCESS_BLOCKED" => return Err(ScraperError::AccessBlocked),
                    "TIMEOUT" => return Err(ScraperError::NetworkTimeout),
                    "NETWORK_ERROR" => return Err(ScraperError::NetworkError),
                    "SERVER_ERROR" => return Err(ScraperError::ServerError),
                    _ => {
                        return Err(ScraperError::Exit {
                            status,
                            stderr: message,
                        });
                    }
                }
            }
        }

        return Err(ScraperError::Exit {
            status,
            stderr: if stderr.is_empty() {
                "no stderr output".to_string()
            } else {
                stderr
            },
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn parse_player_data(json: &str, fallback_slug: &str) -> Result<PlayerData, ScraperError> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let data = value
        .get("data")
        .cloned()
        .unwrap_or_else(|| serde_json::Value::Object(Default::default()));
    let settings = serde_json::from_value::<PlayerSettings>(data)?;

    Ok(PlayerData {
        player: value
            .get("player")
            .and_then(serde_json::Value::as_str)
            .unwrap_or(fallback_slug)
            .to_string(),
        slug: value
            .get("slug")
            .and_then(serde_json::Value::as_str)
            .unwrap_or(fallback_slug)
            .to_string(),
        url: value
            .get("url")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string(),
        data: settings,
    })
}

pub fn fetch_totalcsgo_crosshair(slug: &str) -> Result<Option<TotalCsgoCrosshair>, ScraperError> {
    let url = format!("https://totalcsgo.com/crosshairs/{slug}");
    let html = totalcsgo_client()?
        .get(url)
        .send()?
        .error_for_status()?
        .text()?;
    Ok(parse_totalcsgo_crosshair(&html))
}

fn totalcsgo_client() -> Result<reqwest::blocking::Client, reqwest::Error> {
    reqwest::blocking::Client::builder()
        .timeout(TOTALCSGO_FETCH_TIMEOUT)
        .build()
}

pub fn enrich_with_totalcsgo_crosshair(player: &mut PlayerData, totalcsgo: TotalCsgoCrosshair) {
    player.data.crosshair.import_code = Some(totalcsgo.code);
    if !totalcsgo.command.is_empty() {
        player.data.crosshair.command = Some(totalcsgo.command.clone());

        for command in totalcsgo.command.split(';') {
            let mut parts = command.split_whitespace();
            let Some(key) = parts.next() else { continue };
            let Some(value) = parts.next() else { continue };
            set_crosshair_command_value(&mut player.data.crosshair, key, value);
        }
    }
}

fn parse_totalcsgo_crosshair(html: &str) -> Option<TotalCsgoCrosshair> {
    let document = Html::parse_document(html);
    let body = Selector::parse("body").ok()?;
    let lines: Vec<String> = document
        .select(&body)
        .flat_map(|node| node.text())
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(ToOwned::to_owned)
        .collect();

    parse_totalcsgo_lines(&lines)
}

fn parse_totalcsgo_lines(lines: &[String]) -> Option<TotalCsgoCrosshair> {
    let code = lines
        .windows(2)
        .find(|window| window[0] == "Code" && window[1].starts_with("CSGO-"))
        .map(|window| window[1].clone())?;
    let command = lines
        .windows(2)
        .find(|window| window[0] == "Command" && window[1].starts_with("cl_"))
        .map(|window| window[1].clone())?;

    Some(TotalCsgoCrosshair { code, command })
}

fn set_crosshair_command_value(
    crosshair: &mut crate::models::CrosshairSettings,
    key: &str,
    value: &str,
) {
    let value = Some(value.to_string());
    match key {
        "cl_crosshairstyle" => crosshair.style = value,
        "cl_crosshair_recoil" => crosshair.follow_recoil = value,
        "cl_crosshairdot" => crosshair.dot = value,
        "cl_crosshairsize" => crosshair.size = value,
        "cl_crosshairthickness" => crosshair.thickness = value,
        "cl_crosshairgap" => crosshair.gap = value,
        "cl_crosshair_drawoutline" => crosshair.draw_outline = value,
        "cl_crosshair_outlinethickness" => crosshair.outline_thickness = value,
        "cl_crosshaircolor" => crosshair.color = value,
        "cl_crosshaircolor_r" => crosshair.color_r = value,
        "cl_crosshaircolor_g" => crosshair.color_g = value,
        "cl_crosshaircolor_b" => crosshair.color_b = value,
        "cl_crosshairusealpha" => crosshair.use_alpha = value,
        "cl_crosshairalpha" => crosshair.alpha = value,
        "cl_crosshair_t" => crosshair.t_style = value,
        "cl_crosshairgap_useweaponvalue" => crosshair.deployed_weapon_gap = value,
        "cl_crosshair_dynamic_splitdist" => crosshair.split_distance = value,
        "cl_fixedcrosshairgap" => crosshair.fixed_gap = value,
        "cl_crosshair_dynamic_splitalpha_innermod" => crosshair.inner_split_alpha = value,
        "cl_crosshair_dynamic_splitalpha_outermod" => crosshair.outer_split_alpha = value,
        "cl_crosshair_dynamic_maxdist_splitratio" => crosshair.split_size_ratio = value,
        "cl_crosshair_sniper_width" => crosshair.sniper_width = value,
        _ => {}
    }
}

pub fn player_slug(query: &str) -> Option<String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_lowercase().replace(' ', "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_scraper_output_into_player_data() {
        let json = r#"{
            "player": "XANTARES",
            "slug": "xantares",
            "url": "https://prosettings.net/players/xantares/",
            "data": {
                "mouse": {
                    "dpi": "400",
                    "sensitivity": "2.3"
                },
                "crosshair": {
                    "importCode": "CSGO-abcde-fghij-klmnp-rstuv-wxyz2",
                    "command": "cl_crosshairsize 1.5",
                    "cl_crosshairstyle": "4",
                    "cl_crosshaircolor_r": "50"
                },
                "launchOptions": "-novid -tickrate 128"
            }
        }"#;

        let player = parse_player_data(json, "fallback").unwrap();

        assert_eq!(player.player, "XANTARES");
        assert_eq!(player.slug, "xantares");
        assert_eq!(player.url, "https://prosettings.net/players/xantares/");
        assert_eq!(player.data.mouse.dpi.as_deref(), Some("400"));
        assert_eq!(player.data.mouse.sensitivity.as_deref(), Some("2.3"));
        assert_eq!(
            player.data.crosshair.import_code.as_deref(),
            Some("CSGO-abcde-fghij-klmnp-rstuv-wxyz2")
        );
        assert_eq!(
            player.data.crosshair.command.as_deref(),
            Some("cl_crosshairsize 1.5")
        );
        assert_eq!(player.data.crosshair.style.as_deref(), Some("4"));
        assert_eq!(player.data.crosshair.color_r.as_deref(), Some("50"));
        assert_eq!(
            player.data.launch_options.as_deref(),
            Some("-novid -tickrate 128")
        );
    }

    #[test]
    fn uses_fallbacks_and_defaults_for_missing_fields() {
        let player =
            parse_player_data(r#"{"data":{"mouse":{"dpi":"800"}}}"#, "missing-player").unwrap();

        assert_eq!(player.player, "missing-player");
        assert_eq!(player.slug, "missing-player");
        assert_eq!(player.url, "");
        assert_eq!(player.data.mouse.dpi.as_deref(), Some("800"));
        assert_eq!(player.data.mouse.sensitivity, None);
        assert_eq!(player.data.crosshair.import_code, None);
        assert_eq!(player.data.launch_options, None);
    }

    #[test]
    fn creates_slug_from_query() {
        assert_eq!(player_slug("  s1mple  "), Some("s1mple".to_string()));
        assert_eq!(player_slug("Niko Player"), Some("niko-player".to_string()));
        assert_eq!(player_slug("   "), None);
    }

    #[test]
    fn parses_totalcsgo_current_crosshair_code_and_command() {
        let html = r#"
            <html><body>
                <h2>Current Crosshair</h2>
                <div>Code</div>
                <div>CSGO-FNOLG-fQcPX-V8P7K-VqtAf-ZbJaA</div>
                <div>Command</div>
                <div>cl_crosshaircolor 5; cl_crosshairalpha 255; cl_crosshairgap -3</div>
            </body></html>
        "#;

        let crosshair = parse_totalcsgo_crosshair(html).unwrap();

        assert_eq!(crosshair.code, "CSGO-FNOLG-fQcPX-V8P7K-VqtAf-ZbJaA");
        assert_eq!(
            crosshair.command,
            "cl_crosshaircolor 5; cl_crosshairalpha 255; cl_crosshairgap -3"
        );
    }

    #[test]
    fn totalcsgo_client_uses_a_short_timeout() {
        assert!(TOTALCSGO_FETCH_TIMEOUT <= Duration::from_secs(5));
        assert!(totalcsgo_client().is_ok());
    }

    #[test]
    fn enriches_player_with_totalcsgo_code_command_and_command_values() {
        let mut player =
            parse_player_data(r#"{"data":{"crosshair":{"cl_crosshairgap":"0"}}}"#, "zywoo")
                .unwrap();

        enrich_with_totalcsgo_crosshair(
            &mut player,
            TotalCsgoCrosshair {
                code: "CSGO-FNOLG-fQcPX-V8P7K-VqtAf-ZbJaA".to_string(),
                command: "cl_crosshaircolor 5; cl_crosshairalpha 255; cl_crosshairgap -3"
                    .to_string(),
            },
        );

        assert_eq!(
            player.data.crosshair.import_code.as_deref(),
            Some("CSGO-FNOLG-fQcPX-V8P7K-VqtAf-ZbJaA")
        );
        assert_eq!(
            player.data.crosshair.command.as_deref(),
            Some("cl_crosshaircolor 5; cl_crosshairalpha 255; cl_crosshairgap -3")
        );
        assert_eq!(player.data.crosshair.color.as_deref(), Some("5"));
        assert_eq!(player.data.crosshair.alpha.as_deref(), Some("255"));
        assert_eq!(player.data.crosshair.gap.as_deref(), Some("-3"));
    }
}
