use nexus_adaptive_host::{HandInput, Pose, XrInputFrame};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_stream_activates_right_hand() {
        let frame = pointer_to_input_frame(5, 0.25, 0.5, true);
        assert_eq!(frame.frame, 5);
        assert!(!frame.hands[0].active);
        assert!(frame.hands[1].active);
        assert!(frame.hands[1].grip_pressed);
    }
}
