#![feature(let_else)]

use crate::materials::grid::GridPlugin;
use crate::materials::skybox::SkyboxPlugin;
use crate::utils::{auto_fly_ship, camera_follow_spaceship};
use bevy::prelude::*;

mod materials;
mod utils;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct PlayerShip;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SkyboxPlugin)
        .add_plugin(GridPlugin)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::Update, auto_fly_ship)
        .add_system_to_stage(CoreStage::Update, camera_follow_spaceship)
        .run();
}

fn setup(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let player_ship_entity = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(PlayerShip)
        .id();
    scene_spawner.spawn_as_child(
        asset_server.load("models/spaceship.gltf#Scene0"),
        player_ship_entity,
    );

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(5.0, 0.0, 5.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.5, 1.0, 2.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);
}
