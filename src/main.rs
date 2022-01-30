use crate::skybox::{Skybox, SkyboxPlugin};
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use crate::skybox::mesh::SkyboxMesh;

mod skybox;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct PlayerShip;

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
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0).with_label("fixed_timestep"))
                .with_system(camera_rotate_around_center_point),
        )
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

    // TODO: Move to plugin code
    commands.spawn_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Skybox,
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
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

fn camera_rotate_around_center_point(
    time: Res<Time>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera = q_camera.single_mut();
    let time = time.time_since_startup().as_secs_f32();
    camera.translation = Vec3::new(35.0 * time.cos(), 20.0, 35.0 * time.sin());
    camera.look_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
}

fn camera_follow_spaceship(
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

fn auto_fly_ship(
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
