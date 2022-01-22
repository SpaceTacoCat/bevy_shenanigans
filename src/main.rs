use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct PlayerShip;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::Update, auto_fly_ship)
        .add_system_to_stage(CoreStage::Update, camera_follow_spaceship)
        .run();
}

fn setup(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
) {
    let player_ship_entity = commands.spawn().insert(Transform::default()).insert(GlobalTransform::default()).insert(PlayerShip).id();
    scene_spawner.spawn_as_child(
        asset_server.load("models/spaceship.gltf#Scene0"),
        player_ship_entity,
    );
    commands.spawn_scene(asset_server.load("models/quad.gltf#Scene0"));
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 20.0, -35.0).looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(MainCamera);
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

fn auto_fly_ship(q_spaceship: Query<&Children, With<PlayerShip>>, mut q_transforms: Query<&mut Transform>) {
    let mut ship = if let Ok(children) = q_spaceship.get_single() {
        let child = children.first().unwrap();
        q_transforms.get_mut(*child).unwrap()
    } else { return };

    ship.translation += Vec3::new(0.0, 0.0, 0.1);
}
