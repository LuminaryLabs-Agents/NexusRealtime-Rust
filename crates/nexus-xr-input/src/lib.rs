use nexus_adaptive_host::{HandInput, Pose, XrInputFrame};
use nexus_webxr_adapter::{
    WebXrFramePacket, WebXrInputSourcePacket, WebXrLayerPacket, WebXrPose, WebXrReferenceSpaceType,
    WebXrViewPacket,
};

const LEFT_EYE_VIEWPORT: [u32; 4] = [0, 0, 1440, 1584];
const RIGHT_EYE_VIEWPORT: [u32; 4] = [1440, 0, 1440, 1584];
const IDENTITY_PROJECTION: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];

pub fn pointer_to_input_frame(frame: u64, x: f32, y: f32, pressed: bool) -> XrInputFrame {
    let mut right = HandInput::default();
    right.active = true;
    right.grip_pressed = pressed;
    right.trigger_pressed = pressed;
    right.pose = Pose {
        position: [x, y, -1.0],
        orientation: [0.0, 0.0, 0.0, 1.0],
    };

    XrInputFrame {
        frame,
        predicted_display_time_ns: 0,
        head_pose: Pose::default(),
        hands: [HandInput::default(), right],
    }
}

pub fn synthetic_stereo_hands(frame: u64) -> XrInputFrame {
    let mut left = HandInput::default();
    left.active = true;
    left.pose.position = [-0.25, 1.25, -0.8];
    let mut right = HandInput::default();
    right.active = true;
    right.pose.position = [0.25, 1.25, -0.8];

    XrInputFrame {
        frame,
        predicted_display_time_ns: 0,
        head_pose: Pose::default(),
        hands: [left, right],
    }
}

pub fn pose_to_webxr_pose(pose: Pose) -> WebXrPose {
    WebXrPose {
        position: pose.position,
        orientation: pose.orientation,
    }
}

pub fn hand_to_webxr_input_source(handedness: &str, hand: &HandInput) -> Option<WebXrInputSourcePacket> {
    if !hand.active {
        return None;
    }

    let mut buttons = Vec::new();
    if hand.grip_pressed {
        buttons.push("grip".to_string());
    }
    if hand.trigger_pressed {
        buttons.push("trigger".to_string());
        buttons.push("select".to_string());
    }

    Some(WebXrInputSourcePacket {
        handedness: handedness.to_string(),
        target_ray_mode: "tracked-pointer".to_string(),
        grip_pose: Some(pose_to_webxr_pose(hand.pose)),
        target_ray_pose: Some(pose_to_webxr_pose(hand.pose)),
        profiles: vec!["generic-trigger-squeeze-thumbstick".to_string()],
        buttons,
        axes: vec![hand.thumbstick[0], hand.thumbstick[1]],
    })
}

pub fn input_frame_to_webxr_frame_packet(input: &XrInputFrame, layer: WebXrLayerPacket) -> WebXrFramePacket {
    let views = vec![
        WebXrViewPacket {
            eye: "left".to_string(),
            pose: pose_to_webxr_pose(input.head_pose),
            projection: IDENTITY_PROJECTION,
            viewport: LEFT_EYE_VIEWPORT,
        },
        WebXrViewPacket {
            eye: "right".to_string(),
            pose: pose_to_webxr_pose(input.head_pose),
            projection: IDENTITY_PROJECTION,
            viewport: RIGHT_EYE_VIEWPORT,
        },
    ];

    let mut input_sources = Vec::new();
    if let Some(source) = hand_to_webxr_input_source("left", &input.hands[0]) {
        input_sources.push(source);
    }
    if let Some(source) = hand_to_webxr_input_source("right", &input.hands[1]) {
        input_sources.push(source);
    }

    WebXrFramePacket {
        frame_index: input.frame,
        predicted_display_time_ns: input.predicted_display_time_ns,
        reference_space: WebXrReferenceSpaceType::LocalFloor,
        views,
        input_sources,
        layer,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_layer() -> WebXrLayerPacket {
        WebXrLayerPacket {
            layer_id: "test-projection".to_string(),
            eye_count: 2,
            swapchain_policy: "required-open-xr-projection".to_string(),
            color_format: "rgba8-srgb".to_string(),
            depth_format: Some("depth24".to_string()),
        }
    }

    #[test]
    fn pointer_stream_activates_right_hand() {
        let frame = pointer_to_input_frame(5, 0.25, 0.5, true);
        assert_eq!(frame.frame, 5);
        assert!(!frame.hands[0].active);
        assert!(frame.hands[1].active);
        assert!(frame.hands[1].grip_pressed);
    }

    #[test]
    fn active_hand_becomes_webxr_input_source() {
        let frame = pointer_to_input_frame(7, 0.25, 0.5, true);
        let packet = input_frame_to_webxr_frame_packet(&frame, test_layer());

        assert_eq!(packet.frame_index, 7);
        assert_eq!(packet.reference_space, WebXrReferenceSpaceType::LocalFloor);
        assert_eq!(packet.views.len(), 2);
        assert_eq!(packet.input_sources.len(), 1);
        assert_eq!(packet.input_sources[0].handedness, "right");
        assert!(packet.input_sources[0].buttons.contains(&"grip".to_string()));
        assert!(packet.input_sources[0].buttons.contains(&"select".to_string()));
    }

    #[test]
    fn synthetic_stereo_hands_become_two_webxr_input_sources() {
        let frame = synthetic_stereo_hands(9);
        let packet = input_frame_to_webxr_frame_packet(&frame, test_layer());

        assert_eq!(packet.frame_index, 9);
        assert_eq!(packet.views[0].eye, "left");
        assert_eq!(packet.views[1].eye, "right");
        assert_eq!(packet.input_sources.len(), 2);
        assert_eq!(packet.layer.eye_count, 2);
    }
}
