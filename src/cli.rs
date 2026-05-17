use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::models::PlayerData;
use crate::scraper::player_slug;

#[derive(Debug)]
pub enum Action {
    Query {
        slug: String,
        output: Output,
    },
    Tui {
        pre_search: Option<String>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Output {
    Crosshair,
    Sensitivity,
    Dpi,
    Edpi,
    Mouse,
    Viewmodel,
    Video,
    Radar,
    Hud,
    Launch,
    Gear,
    All,
    Json,
}

impl Output {
    pub fn print(&self, player: &PlayerData) {
        let mut stdout = io::stdout().lock();
        match self {
            Output::Crosshair => {
                if let Some(code) = &player.data.crosshair.import_code {
                    let _ = writeln!(stdout, "{}", code);
                }
            }
            Output::Sensitivity => {
                if let Some(v) = &player.data.mouse.sensitivity {
                    let _ = writeln!(stdout, "{}", v);
                }
            }
            Output::Dpi => {
                if let Some(v) = &player.data.mouse.dpi {
                    let _ = writeln!(stdout, "{}", v);
                }
            }
            Output::Edpi => {
                if let Some(v) = &player.data.mouse.edpi {
                    let _ = writeln!(stdout, "{}", v);
                }
            }
            Output::Mouse => {
                Self::print_field(&mut stdout, "dpi", &player.data.mouse.dpi);
                Self::print_field(&mut stdout, "sensitivity", &player.data.mouse.sensitivity);
                Self::print_field(&mut stdout, "edpi", &player.data.mouse.edpi);
                Self::print_field(
                    &mut stdout,
                    "zoom_sensitivity",
                    &player.data.mouse.zoom_sensitivity_ratio_mouse,
                );
                Self::print_field(&mut stdout, "hz", &player.data.mouse.hz);
                Self::print_field(
                    &mut stdout,
                    "windows_sensitivity",
                    &player.data.mouse.windows_sensitivity,
                );
            }
            Output::Viewmodel => {
                Self::print_field(&mut stdout, "fov", &player.data.viewmodel.fov);
                Self::print_field(&mut stdout, "offset_x", &player.data.viewmodel.offset_x);
                Self::print_field(&mut stdout, "offset_y", &player.data.viewmodel.offset_y);
                Self::print_field(&mut stdout, "offset_z", &player.data.viewmodel.offset_z);
                Self::print_field(
                    &mut stdout,
                    "presetpos",
                    &player.data.viewmodel.presetpos,
                );
                Self::print_field(
                    &mut stdout,
                    "use_new_bob",
                    &player.data.viewmodel.use_new_bob,
                );
            }
            Output::Video => {
                Self::print_field(&mut stdout, "resolution", &player.data.video.resolution);
                Self::print_field(&mut stdout, "aspect_ratio", &player.data.video.aspect_ratio);
                Self::print_field(&mut stdout, "scaling_mode", &player.data.video.scaling_mode);
                Self::print_field(&mut stdout, "brightness", &player.data.video.brightness);
                Self::print_field(&mut stdout, "display_mode", &player.data.video.display_mode);
                Self::print_field(
                    &mut stdout,
                    "boost_contrast",
                    &player.data.video.boost_contrast,
                );
                Self::print_field(&mut stdout, "vsync", &player.data.video.vsync);
                Self::print_field(
                    &mut stdout,
                    "reflex",
                    &player.data.video.reflex_low_latency,
                );
                Self::print_field(&mut stdout, "gsync", &player.data.video.gsync);
                Self::print_field(&mut stdout, "max_fps", &player.data.video.max_fps);
                Self::print_field(
                    &mut stdout,
                    "anti_aliasing",
                    &player.data.video.anti_aliasing,
                );
                Self::print_field(
                    &mut stdout,
                    "shadow_quality",
                    &player.data.video.shadow_quality,
                );
                Self::print_field(
                    &mut stdout,
                    "dynamic_shadows",
                    &player.data.video.dynamic_shadows,
                );
                Self::print_field(
                    &mut stdout,
                    "texture_detail",
                    &player.data.video.texture_detail,
                );
                Self::print_field(
                    &mut stdout,
                    "filtering_mode",
                    &player.data.video.filtering_mode,
                );
                Self::print_field(
                    &mut stdout,
                    "shader_detail",
                    &player.data.video.shader_detail,
                );
                Self::print_field(
                    &mut stdout,
                    "particle_detail",
                    &player.data.video.particle_detail,
                );
                Self::print_field(
                    &mut stdout,
                    "ambient_occlusion",
                    &player.data.video.ambient_occlusion,
                );
                Self::print_field(&mut stdout, "hdr", &player.data.video.hdr);
                Self::print_field(&mut stdout, "fsr", &player.data.video.fsr);
            }
            Output::Radar => {
                Self::print_field(
                    &mut stdout,
                    "always_centered",
                    &player.data.radar.always_centered,
                );
                Self::print_field(&mut stdout, "rotate", &player.data.radar.rotate);
                Self::print_field(
                    &mut stdout,
                    "square_with_scoreboard",
                    &player.data.radar.square_with_scoreboard,
                );
                Self::print_field(&mut stdout, "hud_scale", &player.data.radar.hud_scale);
                Self::print_field(&mut stdout, "scale", &player.data.radar.scale);
            }
            Output::Hud => {
                Self::print_field(&mut stdout, "scaling", &player.data.hud.scaling);
                Self::print_field(&mut stdout, "color", &player.data.hud.color);
            }
            Output::Launch => {
                if let Some(opts) = &player.data.launch_options {
                    let _ = writeln!(stdout, "{}", opts);
                }
            }
            Output::Gear => {
                for item in &player.data.gear {
                    let _ = writeln!(stdout, "{}: {}", item.category.to_lowercase(), item.name);
                }
            }
            Output::All => {
                Output::Mouse.print(player);
                println!();
                Output::Crosshair.print(player);
                Self::print_field(&mut stdout, "crosshair_command", &player.data.crosshair.command);
                Self::print_field(&mut stdout, "crosshair_style", &player.data.crosshair.style);
                Self::print_field(&mut stdout, "crosshair_size", &player.data.crosshair.size);
                Self::print_field(&mut stdout, "crosshair_gap", &player.data.crosshair.gap);
                Self::print_field(
                    &mut stdout,
                    "crosshair_thickness",
                    &player.data.crosshair.thickness,
                );
                Self::print_field(&mut stdout, "crosshair_dot", &player.data.crosshair.dot);
                Self::print_field(
                    &mut stdout,
                    "crosshair_color",
                    &player.data.crosshair.color,
                );
                Self::print_field(&mut stdout, "crosshair_alpha", &player.data.crosshair.alpha);
                Self::print_field(
                    &mut stdout,
                    "crosshair_outline",
                    &player.data.crosshair.draw_outline,
                );
                println!();
                Output::Viewmodel.print(player);
                println!();
                Output::Video.print(player);
                println!();
                Output::Radar.print(player);
                println!();
                Output::Hud.print(player);
                println!();
                Output::Launch.print(player);
            }
            Output::Json => {
                if let Ok(json) = serde_json::to_string_pretty(player) {
                    let _ = writeln!(stdout, "{}", json);
                }
            }
        }
    }

    fn print_field(w: &mut impl Write, label: &str, value: &Option<String>) {
        if let Some(v) = value {
            let _ = writeln!(w, "{}: {}", label, v);
        }
    }
}

pub fn parse_args(args: &[String]) -> Result<Action, String> {
    let mut positional = Vec::new();
    let mut flags = Vec::new();
    let mut output = None;
    let mut help = false;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => help = true,
            "-c" | "--crosshair" => {
                flags.push(arg.clone());
                output = Some(Output::Crosshair);
            }
            "-m" | "--mouse" => {
                flags.push(arg.clone());
                output = Some(Output::Mouse);
            }
            "-s" | "--sensitivity" => {
                flags.push(arg.clone());
                output = Some(Output::Sensitivity);
            }
            "-d" | "--dpi" => {
                flags.push(arg.clone());
                output = Some(Output::Dpi);
            }
            "-e" | "--edpi" => {
                flags.push(arg.clone());
                output = Some(Output::Edpi);
            }
            "-v" | "--viewmodel" => {
                flags.push(arg.clone());
                output = Some(Output::Viewmodel);
            }
            "--video" => {
                flags.push(arg.clone());
                output = Some(Output::Video);
            }
            "-r" | "--radar" => {
                flags.push(arg.clone());
                output = Some(Output::Radar);
            }
            "--hud" => {
                flags.push(arg.clone());
                output = Some(Output::Hud);
            }
            "-l" | "--launch" => {
                flags.push(arg.clone());
                output = Some(Output::Launch);
            }
            "--gear" => {
                flags.push(arg.clone());
                output = Some(Output::Gear);
            }
            "-a" | "--all" => {
                flags.push(arg.clone());
                output = Some(Output::All);
            }
            "-j" | "--json" => {
                flags.push(arg.clone());
                output = Some(Output::Json);
            }
            _ => {
                if !arg.starts_with('-') {
                    positional.push(arg.clone());
                }
            }
        }
    }

    if help {
        return Err(HELP_TEXT.to_string());
    }

    if flags.len() > 1 {
        return Err(format!(
            "Only one flag allowed at a time. Got: {}",
            flags.join(", ")
        ));
    }

    let player_name = positional.first().cloned();

    if let Some(output) = output {
        let slug = player_name
            .as_deref()
            .and_then(player_slug)
            .ok_or_else(|| "Player name required. Usage: csprotui <player> --flag".to_string())?;
        Ok(Action::Query { slug, output })
    } else {
        Ok(Action::Tui {
            pre_search: player_name,
        })
    }
}

const HELP_TEXT: &str = "\
CSPROTUI — Terminal-based CS2 pro player stats viewer

Usage:
  csprotui [<player>] [flag]

Flags:
  -c, --crosshair     Crosshair import code (CSGO-xxx)
  -s, --sensitivity   Mouse sensitivity
  -d, --dpi           Mouse DPI
  -e, --edpi          Mouse eDPI
  -m, --mouse         All mouse settings
  -v, --viewmodel     Viewmodel settings
  --video             Video settings
  -r, --radar         Radar settings
  --hud               HUD settings
  -l, --launch        Launch options
  --gear              Gear (monitor, mouse, keyboard, etc.)
  -a, --all           All settings
  -j, --json          Full JSON output
  -h, --help          Show this help

Examples:
  csprotui xantares --crosshair
  csprotui s1mple -m
  csprotui zywoo --all
  csprotui donk --json | jq .data.mouse";

// --- Spinner ---

pub fn with_spinner(label: &str, f: impl FnOnce()) {
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = Arc::clone(&done);

    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let label = label.to_string();

    let handle = thread::spawn(move || {
        let mut stderr = io::stderr();
        let mut i = 0usize;
        while !done_clone.load(Ordering::Relaxed) {
            let _ = write!(
                stderr,
                "\r{} Searching {}...",
                spinner_chars[i % spinner_chars.len()],
                label
            );
            let _ = stderr.flush();
            i += 1;
            thread::sleep(Duration::from_millis(100));
        }
        // Clear the line
        let _ = write!(stderr, "\r\x1b[K");
        let _ = stderr.flush();
    });

    f();

    done.store(true, Ordering::Relaxed);
    let _ = handle.join();
}
