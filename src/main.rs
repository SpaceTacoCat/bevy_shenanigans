use crate::skybox::{SkyboxMaterial, SkyboxPlugin};
use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod skybox;
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
        .add_startup_system(setup)
        // .add_stage_after(
        //     CoreStage::Update,
        //     FixedUpdateStage,
        //     SystemStage::parallel()
        //         .with_run_criteria(FixedTimestep::step(1.0 / 60.0).with_label("fixed_timestep"))
        //         .with_system(utils::camera_rotate_around_center_point),
        // )
        // .add_system_to_stage(CoreStage::Update, auto_fly_ship)
        // .add_system_to_stage(CoreStage::Update, camera_follow_spaceship)
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

    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        GlobalTransform::default(),
        SkyboxMaterial,
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(5.0, 0.0, 5.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 20.0, -35.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);
}
