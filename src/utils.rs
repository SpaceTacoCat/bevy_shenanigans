#![allow(dead_code)]

use crate::{MainCamera, PlayerShip};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub fn camera_follow_spaceship(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    q_spaceship: Query<&Children, With<PlayerShip>>,
    q_transforms: Query<&Transform, Without<MainCamera>>,
) {
    let mut camera = if let Ok(camera) = q_camera.get_single_mut() {
        camera
    } else {
        return;
    };
    let spaceship = if let Ok(children) = q_spaceship.get_single() {
        children.first().unwrap()
    } else {
        return;
    };

    let ship_object = q_transforms.get(*spaceship).unwrap();

    camera.translation = ship_object.translation + Vec3::new(0.0, 20.0, -35.0);
    camera.look_at(ship_object.translation + Vec3::Y * 5.0, Vec3::Y);
}

pub fn auto_fly_ship(
    q_spaceship: Query<&Children, With<PlayerShip>>,
    mut q_transforms: Query<&mut Transform>,
) {
    let mut ship = if let Ok(children) = q_spaceship.get_single() {
        let child = children.first().unwrap();
        q_transforms.get_mut(*child).unwrap()
    } else {
        return;
    };

    ship.translation += Vec3::new(0.0, 0.0, 0.1);
}

pub fn rotate_camera_with_mouse(
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
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
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera = q_camera.single_mut();
    let fwd = camera.forward();
    for event in mouse_wheel.iter() {
        camera.translation += fwd * -event.y.signum();
    }
}
