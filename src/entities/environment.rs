use crate::utils::alter_transform_once::AlterTransformOnce;
use crate::utils::spawn;
use bevy::prelude::{
    AssetServer, Commands, GlobalTransform, Res, ResMut, SceneSpawner, Transform, Vec3,
};

pub fn spawn_sample_scene(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
) {
    spawn::spawn_model_override(
        "models/pyramid.gltf",
        &mut commands,
        scene_spawner.as_mut(),
        asset_server.as_ref(),
        |manifest| manifest.transform.translation.z += 2500.0,
    );

    let ent = commands
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            AlterTransformOnce {
                target: Transform::from_translation(Vec3::new(10.0, 2.0, 10.0)),
            },
        ))
        .id();

    scene_spawner.spawn_as_child(asset_server.load("models/pillar.gltf#Scene0"), ent);
}
