use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HostMode {
    AndroidCanvas,
    StereoPanel,
    QuestOpenXr,
    Headless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwapchainPolicy {
    NotRequired,
    OptionalStereoLayer,
    RequiredOpenXrProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputStreamPolicy {
    PointerOnly,
    SyntheticStereoHands,
    OpenXrActions,
    HeadlessReplay,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveHostProfile {
    pub id: String,
    pub mode: HostMode,
    pub stereo: bool,
    pub swapchain: SwapchainPolicy,
    pub input: InputStreamPolicy,
    pub target_frame_rate: u32,
    pub render_scale: f32,
    pub kits: Vec<String>,
}

impl AdaptiveHostProfile {
    pub fn needs_openxr_swapchain(&self) -> bool {
        self.swapchain == SwapchainPolicy::RequiredOpenXrProjection
    }

    pub fn needs_pose_stream(&self) -> bool {
        matches!(self.input, InputStreamPolicy::OpenXrActions | InputStreamPolicy::SyntheticStereoHands)
    }

    pub fn quest_openxr() -> Self {
        Self {
            id: "quest-openxr".to_string(),
            mode: HostMode::QuestOpenXr,
            stereo: true,
            swapchain: SwapchainPolicy::RequiredOpenXrProjection,
            input: InputStreamPolicy::OpenXrActions,
            target_frame_rate: 72,
            render_scale: 1.0,
            kits: vec![
                "xr-input-kit".to_string(),
                "xr-grab-throw-kit".to_string(),
                "simple-rigid-body-kit".to_string(),
                "toon-visual-kit".to_string(),
                "sky-gradient-kit".to_string(),
            ],
        }
    }

    pub fn android_canvas() -> Self {
        Self {
            id: "android-canvas".to_string(),
            mode: HostMode::AndroidCanvas,
            stereo: false,
            swapchain: SwapchainPolicy::NotRequired,
            input: InputStreamPolicy::PointerOnly,
            target_frame_rate: 60,
            render_scale: 1.0,
            kits: vec![
                "xr-grab-throw-kit".to_string(),
                "toon-visual-kit".to_string(),
                "sky-gradient-kit".to_string(),
            ],
        }
    }

    pub fn stereo_panel() -> Self {
        Self {
            id: "stereo-panel".to_string(),
            mode: HostMode::StereoPanel,
            stereo: true,
            swapchain: SwapchainPolicy::OptionalStereoLayer,
            input: InputStreamPolicy::SyntheticStereoHands,
            target_frame_rate: 72,
            render_scale: 0.85,
            kits: vec![
                "xr-input-kit".to_string(),
                "xr-grab-throw-kit".to_string(),
                "toon-visual-kit".to_string(),
                "sky-gradient-kit".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XrInputFrame {
    pub frame: u64,
    pub predicted_display_time_ns: u64,
    pub head_pose: Pose,
    pub hands: [HandInput; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub position: [f32; 3],
    pub orientation: [f32; 4],
}

impl Default for Pose {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            orientation: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HandInput {
    pub active: bool,
    pub pose: Pose,
    pub grip_pressed: bool,
    pub trigger_pressed: bool,
    pub thumbstick: [f32; 2],
}

impl Default for HandInput {
    fn default() -> Self {
        Self {
            active: false,
            pose: Pose::default(),
            grip_pressed: false,
            trigger_pressed: false,
            thumbstick: [0.0, 0.0],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrabInteractionDescriptor {
    pub object_id: String,
    pub mode: String,
    pub radius: f32,
    pub mass: f32,
    pub throw_scale: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StereoRenderDescriptor {
    pub eye_count: u8,
    pub projection_source: String,
    pub cel_bands: u8,
    pub outline_mode: String,
    pub sky_mode: String,
}

impl Default for StereoRenderDescriptor {
    fn default() -> Self {
        Self {
            eye_count: 2,
            projection_source: "host-view".to_string(),
            cel_bands: 4,
            outline_mode: "sigmoid-depth-normal".to_string(),
            sky_mode: "gradient-horizon".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_profile_requires_swapchain_and_pose_stream() {
        let profile = AdaptiveHostProfile::quest_openxr();
        assert!(profile.needs_openxr_swapchain());
        assert!(profile.needs_pose_stream());
        assert_eq!(profile.stereo, true);
    }

    #[test]
    fn canvas_profile_does_not_require_swapchain() {
        let profile = AdaptiveHostProfile::android_canvas();
        assert!(!profile.needs_openxr_swapchain());
        assert!(!profile.needs_pose_stream());
    }
}
