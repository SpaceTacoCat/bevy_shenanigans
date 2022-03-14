use crate::utils::alter_transform_once::AlterTransformOnce;
use crate::utils::spawn;
use bevy::prelude::{
    AssetServer, Commands, GlobalTransform, Res, ResMut, SceneSpawner, Transform, Vec3,
};
use bevy_rapier3d::na::{DMatrix, Vector3};
use bevy_rapier3d::prelude::{ColliderBundle, ColliderShape, RigidBodyBundle, RigidBodyType};

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

    // Spawn ground plane
    commands
        .spawn()
        .insert(Transform {
            translation: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::heightfield(DMatrix::zeros(2, 2), Vector3::new(1e9, 1.0, 1e9))
                .into(),
            ..Default::default()
        });

    for i in 1..10000 {
        let ent = commands
            .spawn()
            .insert_bundle((
                Transform::default(),
                GlobalTransform::default(),
                AlterTransformOnce {
                    target: Transform::from_translation(Vec3::new(10.0, 2.0, 10.0 * (i as f32))),
                },
            ))
            .id();

        scene_spawner.spawn_as_child(asset_server.load("models/pillar.gltf#Scene0"), ent);
    }
}
