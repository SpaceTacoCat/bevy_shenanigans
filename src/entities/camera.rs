use crate::entities::ship::{PlayerShipMarker, LABEL_FLY_SHIP};
use crate::{App, MainCameraMarker};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_main_camera)
            .add_system(camera_follow_spaceship.after(LABEL_FLY_SHIP));
    }
}

fn setup_main_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainCameraMarker);
}

pub fn camera_follow_spaceship(
    mut q_camera: Query<&mut Transform, With<MainCameraMarker>>,
    q_spaceship: Query<&Transform, (With<PlayerShipMarker>, Without<MainCameraMarker>)>,
) {
    let Ok(mut camera) = q_camera.get_single_mut() else {
        return;
    };
    let Ok(spaceship) = q_spaceship.get_single() else {
        return;
    };

    camera.translation = spaceship.translation + Vec3::new(0.0, 15.0, -40.0);
    camera.look_at(spaceship.translation + Vec3::Y * 10.0, Vec3::Y);
}
