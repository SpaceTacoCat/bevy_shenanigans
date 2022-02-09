use crate::utils::alter_transform_once::AlterTransformOnce;
use bevy::asset::AssetPath;
use bevy::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Default)]
struct Manifest {
    transform: ManifestTransform,
}

#[derive(Deserialize, Default)]
struct ManifestTransform {
    translation: MVec3,
    scale: MVec3,
}

#[derive(Deserialize, Default)]
struct MVec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Manifest {
    fn to_alter_transform_once(&self) -> AlterTransformOnce {
        AlterTransformOnce {
            target: Transform::from_translation(Vec3::new(
                self.transform.translation.x,
                self.transform.translation.y,
                self.transform.translation.z,
            ))
            .with_scale(Vec3::new(
                self.transform.scale.x,
                self.transform.scale.y,
                self.transform.scale.z,
            )),
        }
    }
}

pub fn spawn_pillar(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
) {
    spawn_model_override(
        "models/pyramid.gltf",
        &mut commands,
        scene_spawner.as_mut(),
        asset_server.as_ref(),
        |manifest| manifest.transform.translation.z += 500.0,
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

/// Spawn given `gltf` asset with optional manifest
fn spawn_model_override<'a>(
    path: impl AsRef<Path>,
    commands: &'a mut Commands,
    scene_spawner: &mut SceneSpawner,
    asset_server: &AssetServer,
    manifest_override: impl FnOnce(&mut Manifest),
) -> Entity {
    let gltf_path = path.as_ref();
    let manifest_path = path.as_ref().with_extension("json");

    let mut manifest: Manifest = match File::open(PathBuf::from("assets").join(&manifest_path)) {
        Ok(f) => serde_json::from_reader(f).unwrap(),
        Err(e) => {
            info!("Failed to open manifest file `{:?}`: {}", manifest_path, e);
            Manifest::default()
        }
    };

    manifest_override(&mut manifest);

    let alter_transform_once = manifest.to_alter_transform_once();

    let ent = commands
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            alter_transform_once,
        ))
        .id();

    scene_spawner.spawn_as_child(
        asset_server.load(AssetPath::new_ref(gltf_path, Some("Scene0"))),
        ent,
    );

    ent
}
