use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion},
    prelude::*,
    window::CursorMoved,
};
use crate::components::*;
use crate::utilities::*;

#[derive(Default)]
pub struct LocalPlayerCameraState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

const MOUSE_SENSITIVITY: f32 = 1.0;
const MAX_PITCH_ANGLE: f32 = 89.0;

/// This system prints out all mouse events as they come in
pub fn local_player_camera_system(
    mut state: ResMut<LocalPlayerCameraState>,
    time: Res<Time>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut player_head_query: Query<(&LocalPlayerHead, &mut Rotation)>,
    mut player_body_query: Query<(&LocalPlayerBody, &mut Rotation)>,
) {
    let motion = {
        let mut m_motion_x = 0.0;
        let mut m_motion_y = 0.0;

        for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
            m_motion_x = event.delta.y() * -1.0;
            m_motion_y = event.delta.x() * -1.0;
        }

        (
            m_motion_x * MOUSE_SENSITIVITY,
            m_motion_y * MOUSE_SENSITIVITY
        )
    };

    for (_player_body, mut rotation) in &mut player_body_query.iter() {
        let delta_rotation_yaw = Quat::from_axis_angle(Vec3::unit_y(), motion.1 * time.delta_seconds);

        rotation.0 = rotation.0 * delta_rotation_yaw;
    }

    for (_player_head, mut rotation) in &mut player_head_query.iter() {
        let pitch_clamper = {   
            let angles = rotation.0.ypr();
            let mut pitch_deg = angles.0.to_degrees();

            if angles.0.abs() > std::f32::consts::FRAC_PI_2 {
                // Invert the pitch
                if pitch_deg < 0.0 {
                    pitch_deg = pitch_deg + 180.0;
                } else {
                    pitch_deg = pitch_deg - 180.0;
                }
            }
            if pitch_deg > MAX_PITCH_ANGLE || pitch_deg < -MAX_PITCH_ANGLE {
                if pitch_deg.signum() != motion.0.signum() {
                    1.0
                } else {
                    0.0
                }
            } else {
                1.0
            }
        };

        let delta_rotation_pitch = Quat::from_axis_angle(Vec3::unit_x(), motion.0 * pitch_clamper * time.delta_seconds);

        rotation.0 = rotation.0 * delta_rotation_pitch;
    }
}
