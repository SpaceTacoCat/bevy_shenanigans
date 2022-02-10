#![feature(let_else)]
#![feature(path_try_exists)]

use crate::entities::environment::spawn_sample_scene;
use crate::entities::ship::ShipAndControlPlugin;
use crate::materials::skybox::SkyboxPlugin;
use crate::materials::vignette::VignetteShaderPlugin;
use crate::utils::alter_transform_once::init_translation;
use crate::utils::local_settings::LocalSettingsPlugin;
use bevy::prelude::shape::{Cube, Plane};
use bevy::prelude::*;
use bevy_easings::EasingsPlugin;
use bevy_rapier3d::prelude::*;

mod entities;
mod materials;
mod utils;

#[derive(Component)]
pub struct MainCameraMarker;

fn main() {
    let mut app = App::new();

    app.insert_resource(AmbientLight {
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
    // .add_plugin(VignetteShaderPlugin)
    .add_startup_system(setup)
    .add_startup_system(spawn_sample_scene)
    .add_system(init_translation);

    // bevy_mod_debugdump::print_render_graph(&mut app);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube { size: 1.0 })),
        transform: Transform::from_xyz(5.0, 0.0, 5.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane { size: 1000.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.8, 0.8),
            ..Default::default()
        }),
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            perspective_projection: PerspectiveProjection {
                far: 5000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainCameraMarker);
}
