#![feature(let_else)]
#![feature(path_try_exists)]
#![feature(generic_const_exprs)]

use crate::entities::environment::spawn_sample_scene;
use crate::entities::ship::ShipControlPlugin;
use crate::materials::skybox::SkyboxPlugin;
use crate::utils::alter_transform_once::AlterTransformOncePlugin;
use crate::utils::local_settings::LocalSettingsPlugin;
use bevy::prelude::*;
use bevy::DefaultPlugins;

use crate::entities::camera::CameraPlugin;
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
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierRenderPlugin)
    .add_plugin(EasingsPlugin)
    .add_plugin(LocalSettingsPlugin {
        filename: "settings.json".to_string(),
    })
    .add_plugin(SkyboxPlugin)
    .add_plugin(ShipControlPlugin)
    .add_plugin(CameraPlugin)
    .add_plugin(AlterTransformOncePlugin)
    // .add_plugin(VignetteShaderPlugin)
    .add_startup_system(spawn_sample_scene);

    // bevy_mod_debugdump::print_render_graph(&mut app);
    app.run();
}
