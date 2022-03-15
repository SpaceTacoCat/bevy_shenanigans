#![allow(dead_code)]

use crate::MainCameraMarker;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::{
    EulerRot, EventReader, Input, Local, MouseButton, Quat, Query, Res, Transform, Vec2, Windows,
    With,
};

pub mod alter_transform_once;
pub mod local_settings;
pub mod macros;
pub mod spawn;

pub fn rotate_camera_with_mouse(
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut q_camera: Query<&mut Transform, With<MainCameraMarker>>,
    mut position: Local<Vec2>,
) {
    let win = windows.get_primary().unwrap();
    if mouse_button.just_pressed(MouseButton::Right) {
        *position = win.cursor_position().unwrap();
    } else if mouse_button.pressed(MouseButton::Right) {
        let last_position = *position;
        let new_position = win.cursor_position().unwrap();
        *position = new_position;

        let delta = new_position - last_position;

        let mut camera = q_camera.single_mut();

        camera.rotation =
            camera
                .rotation
                .mul_quat(Quat::from_euler(EulerRot::XYZ, 0.0, -delta.x / 200.0, 0.0));
        camera.rotation =
            camera
                .rotation
                .mul_quat(Quat::from_euler(EulerRot::XYZ, delta.y / 200.0, 0.0, 0.0));
    }
}

pub fn move_camera_with_wheel(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut q_camera: Query<&mut Transform, With<MainCameraMarker>>,
) {
    let mut camera = q_camera.single_mut();
    let fwd = camera.forward();
    for event in mouse_wheel.iter() {
        camera.translation += fwd * -event.y.signum();
    }
}
