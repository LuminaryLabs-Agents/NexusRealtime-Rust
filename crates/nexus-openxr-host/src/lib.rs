use nexus_adaptive_host::{AdaptiveHostProfile, SwapchainPolicy};
use nexus_webxr_adapter::{WebXrHostCapabilities, WebXrLayerPacket, WebXrSessionRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenXrBootStage {
    NotStarted,
    LoaderReady,
    InstanceReady,
    SessionReady,
    SwapchainsReady,
    FrameLoopReady,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenXrHostPlan {
    pub app_name: String,
    pub profile: AdaptiveHostProfile,
    pub stage: OpenXrBootStage,
    pub eye_count: u8,
}

impl OpenXrHostPlan {
    pub fn quest_house_demo() -> Self {
        Self {
            app_name: "NexusRealtime XR House".to_string(),
            profile: AdaptiveHostProfile::quest_openxr(),
            stage: OpenXrBootStage::NotStarted,
            eye_count: 2,
        }
    }

    pub fn advance_to(&mut self, stage: OpenXrBootStage) {
        self.stage = stage;
    }

    pub fn requires_projection_swapchain(&self) -> bool {
        self.profile.swapchain == SwapchainPolicy::RequiredOpenXrProjection && self.eye_count == 2
    }

    pub fn webxr_capabilities(&self) -> WebXrHostCapabilities {
        WebXrHostCapabilities::quest_openxr_vulkan()
    }

    pub fn default_session_request(&self) -> WebXrSessionRequest {
        WebXrSessionRequest::immersive_vr_local_floor()
    }

    pub fn projection_layer_packet(&self) -> WebXrLayerPacket {
        WebXrLayerPacket {
            layer_id: "primary-projection".to_string(),
            eye_count: self.eye_count,
            swapchain_policy: "required-open-xr-projection".to_string(),
            color_format: "rgba8-srgb".to_string(),
            depth_format: Some("depth24".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EyeViewPlan {
    pub eye_index: u8,
    pub recommended_width: u32,
    pub recommended_height: u32,
    pub near_z: f32,
    pub far_z: f32,
}

pub fn default_eye_views() -> [EyeViewPlan; 2] {
    [
        EyeViewPlan { eye_index: 0, recommended_width: 1440, recommended_height: 1584, near_z: 0.05, far_z: 100.0 },
        EyeViewPlan { eye_index: 1, recommended_width: 1440, recommended_height: 1584, near_z: 0.05, far_z: 100.0 },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_plan_requires_two_eye_swapchain() {
        let plan = OpenXrHostPlan::quest_house_demo();
        assert!(plan.requires_projection_swapchain());
        assert_eq!(default_eye_views().len(), 2);
    }

    #[test]
    fn quest_plan_exposes_webxr_capabilities() {
        let plan = OpenXrHostPlan::quest_house_demo();
        assert!(plan.webxr_capabilities().supports_request(&plan.default_session_request()));
        assert_eq!(plan.projection_layer_packet().eye_count, 2);
    }
}
