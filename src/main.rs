use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct PlayerShip;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(camera_follow_spaceship)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("models/spaceship.gltf#Scene0"));
    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .insert(MainCamera);
    // const HALF_SIZE: f32 = 1.0;
    // commands.spawn_bundle(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadow_projection: OrthographicProjection {
    //             left: -HALF_SIZE,
    //             right: HALF_SIZE,
    //             bottom: -HALF_SIZE,
    //             top: HALF_SIZE,
    //             near: -10.0 * HALF_SIZE,
    //             far: 10.0 * HALF_SIZE,
    //             ..Default::default()
    //         },
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
}
