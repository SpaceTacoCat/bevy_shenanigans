use crate::{MainCamera, PlayerShip};
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
