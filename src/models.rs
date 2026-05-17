use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub player: String,
    pub slug: String,
    pub url: String,
    pub data: PlayerSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSettings {
    #[serde(default)]
    pub mouse: MouseSettings,
    #[serde(default)]
    pub crosshair: CrosshairSettings,
    #[serde(default)]
    pub viewmodel: ViewmodelSettings,
    #[serde(default)]
    pub video: VideoSettings,
    #[serde(default)]
    pub radar: RadarSettings,
    #[serde(default)]
    pub hud: HudSettings,
    #[serde(default)]
    pub bob: BobSettings,
    #[serde(rename = "launchOptions")]
    #[serde(default)]
    pub launch_options: Option<String>,
    #[serde(default)]
    pub gear: Vec<GearItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MouseSettings {
    pub dpi: Option<String>,
    pub sensitivity: Option<String>,
    pub edpi: Option<String>,
    pub zoom_sensitivity_ratio_mouse: Option<String>,
    pub hz: Option<String>,
    pub windows_sensitivity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrosshairSettings {
    #[serde(rename = "importCode")]
    pub import_code: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(rename = "cl_crosshairstyle")]
    pub style: Option<String>,
    #[serde(rename = "cl_crosshair_recoil")]
    pub follow_recoil: Option<String>,
    #[serde(rename = "cl_crosshairdot")]
    pub dot: Option<String>,
    #[serde(rename = "cl_crosshairsize")]
    pub size: Option<String>,
    #[serde(rename = "cl_crosshairthickness")]
    pub thickness: Option<String>,
    #[serde(rename = "cl_crosshairgap")]
    pub gap: Option<String>,
    #[serde(rename = "cl_crosshair_drawoutline")]
    pub draw_outline: Option<String>,
    #[serde(rename = "cl_crosshair_outlinethickness")]
    pub outline_thickness: Option<String>,
    #[serde(rename = "cl_crosshaircolor")]
    pub color: Option<String>,
    #[serde(rename = "cl_crosshaircolor_r")]
    pub color_r: Option<String>,
    #[serde(rename = "cl_crosshaircolor_g")]
    pub color_g: Option<String>,
    #[serde(rename = "cl_crosshaircolor_b")]
    pub color_b: Option<String>,
    #[serde(rename = "cl_crosshairusealpha")]
    pub use_alpha: Option<String>,
    #[serde(rename = "cl_crosshairalpha")]
    pub alpha: Option<String>,
    #[serde(rename = "cl_crosshair_t")]
    pub t_style: Option<String>,
    #[serde(rename = "cl_crosshairgap_useweaponvalue")]
    pub deployed_weapon_gap: Option<String>,
    #[serde(rename = "cl_crosshair_dynamic_splitdist")]
    pub split_distance: Option<String>,
    #[serde(rename = "cl_fixedcrosshairgap")]
    pub fixed_gap: Option<String>,
    #[serde(rename = "cl_crosshair_dynamic_splitalpha_innermod")]
    pub inner_split_alpha: Option<String>,
    #[serde(rename = "cl_crosshair_dynamic_splitalpha_outermod")]
    pub outer_split_alpha: Option<String>,
    #[serde(rename = "cl_crosshair_dynamic_maxdist_splitratio")]
    pub split_size_ratio: Option<String>,
    #[serde(rename = "cl_crosshair_sniper_width")]
    pub sniper_width: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ViewmodelSettings {
    #[serde(rename = "viewmodel_fov")]
    pub fov: Option<String>,
    #[serde(rename = "viewmodel_offset_x")]
    pub offset_x: Option<String>,
    #[serde(rename = "viewmodel_offset_y")]
    pub offset_y: Option<String>,
    #[serde(rename = "viewmodel_offset_z")]
    pub offset_z: Option<String>,
    #[serde(rename = "viewmodel_presetpos")]
    pub presetpos: Option<String>,
    #[serde(rename = "cl_usenewbob")]
    pub use_new_bob: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VideoSettings {
    pub resolution: Option<String>,
    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "scaling_mode")]
    pub scaling_mode: Option<String>,
    pub brightness: Option<String>,
    #[serde(rename = "display_mode")]
    pub display_mode: Option<String>,
    #[serde(rename = "boost_player_contrast")]
    pub boost_contrast: Option<String>,
    #[serde(rename = "wait_for_vertical_sync")]
    pub vsync: Option<String>,
    #[serde(rename = "nvidia_reflex_low_latency")]
    pub reflex_low_latency: Option<String>,
    #[serde(rename = "nvidia_g-sync")]
    pub gsync: Option<String>,
    #[serde(rename = "max_fps")]
    pub max_fps: Option<String>,
    #[serde(rename = "multisampling_anti-aliasing_mode")]
    pub anti_aliasing: Option<String>,
    #[serde(rename = "global_shadow_quality")]
    pub shadow_quality: Option<String>,
    #[serde(rename = "dynamic_shadows")]
    pub dynamic_shadows: Option<String>,
    #[serde(rename = "model_texture_detail")]
    pub texture_detail: Option<String>,
    #[serde(rename = "texture_filtering_mode")]
    pub filtering_mode: Option<String>,
    #[serde(rename = "shader_detail")]
    pub shader_detail: Option<String>,
    #[serde(rename = "particle_detail")]
    pub particle_detail: Option<String>,
    #[serde(rename = "ambient_occlusion")]
    pub ambient_occlusion: Option<String>,
    #[serde(rename = "high_dynamic_range")]
    pub hdr: Option<String>,
    #[serde(rename = "fidelityfx_super_resolution")]
    pub fsr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RadarSettings {
    #[serde(rename = "cl_radar_always_centered")]
    pub always_centered: Option<String>,
    #[serde(rename = "cl_radar_rotate")]
    pub rotate: Option<String>,
    #[serde(rename = "cl_radar_square_with_scoreboard")]
    pub square_with_scoreboard: Option<String>,
    #[serde(rename = "cl_radar_hud_scale")]
    pub hud_scale: Option<String>,
    #[serde(rename = "cl_radar_scale")]
    pub scale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HudSettings {
    #[serde(rename = "hud_scaling")]
    pub scaling: Option<String>,
    #[serde(rename = "cl_hud_color")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BobSettings {
    #[serde(rename = "cl_bob_lower_amt")]
    pub lower_amt: Option<String>,
    #[serde(rename = "cl_bobamt_lat")]
    pub amt_lat: Option<String>,
    #[serde(rename = "cl_bobamt_vert")]
    pub amt_vert: Option<String>,
    #[serde(rename = "cl_bobcycle")]
    pub cycle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GearItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub category: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tab {
    Mouse,
    Crosshair,
    Viewmodel,
    Video,
    Radar,
    Hud,
    Gear,
    LaunchOptions,
}

impl Tab {
    pub const ALL: [Self; 8] = [
        Self::Mouse,
        Self::Crosshair,
        Self::Viewmodel,
        Self::Video,
        Self::Radar,
        Self::Hud,
        Self::Gear,
        Self::LaunchOptions,
    ];

    pub fn next(&mut self) {
        let next = (self.index() + 1) % Self::ALL.len();
        *self = Self::ALL[next];
    }

    pub fn prev(&mut self) {
        let next = self.index().checked_sub(1).unwrap_or(Self::ALL.len() - 1);
        *self = Self::ALL[next];
    }

    pub fn index(&self) -> usize {
        Self::ALL.iter().position(|tab| tab == self).unwrap_or(0)
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Tab::Mouse => "Mouse",
            Tab::Crosshair => "Crosshair",
            Tab::Viewmodel => "Viewmodel",
            Tab::Video => "Video",
            Tab::Radar => "Radar",
            Tab::Hud => "HUD",
            Tab::Gear => "Gear",
            Tab::LaunchOptions => "Launch Options",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Tab::LaunchOptions => "Launch",
            _ => self.name(),
        }
    }
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::Tab;

    #[test]
    fn tab_index_and_from_index_share_the_same_order() {
        for (index, tab) in Tab::ALL.iter().enumerate() {
            assert_eq!(tab.index(), index);
            assert_eq!(Tab::from_index(index), Some(*tab));
        }
        assert_eq!(Tab::from_index(Tab::ALL.len()), None);
    }

    #[test]
    fn tab_next_and_prev_wrap() {
        let mut tab = Tab::Mouse;
        tab.prev();
        assert_eq!(tab, Tab::LaunchOptions);
        tab.next();
        assert_eq!(tab, Tab::Mouse);

        for expected in [
            Tab::Crosshair,
            Tab::Viewmodel,
            Tab::Video,
            Tab::Radar,
            Tab::Hud,
            Tab::LaunchOptions,
        ] {
            tab.next();
            assert_eq!(tab, expected);
        }
    }

    #[test]
    fn launch_tab_keeps_short_label() {
        assert_eq!(Tab::LaunchOptions.name(), "Launch Options");
        assert_eq!(Tab::LaunchOptions.short_name(), "Launch");
    }
}
