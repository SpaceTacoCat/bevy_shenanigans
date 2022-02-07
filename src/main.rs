#![feature(let_else)]
#![feature(path_try_exists)]

use crate::entities::ship::ShipAndControlPlugin;
use crate::materials::skybox::SkyboxPlugin;
use crate::utils::local_settings::LocalSettingsPlugin;
use bevy::prelude::*;
use bevy_easings::EasingsPlugin;
use bevy_rapier3d::prelude::*;

mod entities;
mod materials;
mod utils;

#[derive(Component)]
pub struct MainCameraMarker;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LocalSettingsPlugin {
            filename: "settings.json".to_string(),
        })
        .add_plugin(SkyboxPlugin)
        .add_plugin(ShipAndControlPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(EasingsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(5.0, 0.0, 5.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .insert(MainCameraMarker);
}
