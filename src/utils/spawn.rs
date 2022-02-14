use crate::utils::alter_transform_once::AlterTransformOnce;
use bevy::asset::AssetPath;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Default)]
pub struct Manifest {
    pub transform: ManifestTransform,
}

#[derive(Deserialize, Default)]
pub struct ManifestTransform {
    pub translation: MVec3,
    pub scale: MVec3,
}

#[derive(Deserialize, Default)]
pub struct MVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Manifest {
    fn from_path_or_default(manifest_path: &PathBuf) -> Self {
        match File::open(PathBuf::from("assets").join(&manifest_path)) {
            Ok(f) => serde_json::from_reader(f).unwrap(),
            Err(e) => {
                debug!("Couldn't find manifest file `{:?}`: {}", manifest_path, e);
                Manifest::default()
            }
        }
    }

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

/// Simple spawn for model with already existing entity
pub fn spawn_model_as_child(
    path: impl AsRef<Path>,
    scene_spawner: &mut SceneSpawner,
    asset_server: &AssetServer,
    entity_commands: &mut EntityCommands,
) -> Entity {
    let gltf_path = path.as_ref();

    let ent = entity_commands.id();

    scene_spawner.spawn_as_child(
        asset_server.load(AssetPath::new_ref(gltf_path, Some("Scene0"))),
        ent,
    );

    ent
}

/// Spawn given `gltf` asset with optional manifest
pub fn spawn_model(
    path: impl AsRef<Path>,
    commands: &mut Commands,
    scene_spawner: &mut SceneSpawner,
    asset_server: &AssetServer,
) -> Entity {
    let gltf_path = path.as_ref();
    let manifest_path = path.as_ref().with_extension("json");

    let manifest = Manifest::from_path_or_default(&manifest_path);

    spawn_with_manifest(commands, scene_spawner, asset_server, gltf_path, &manifest)
}

/// Spawn given `gltf` asset with optional manifest and override function
pub fn spawn_model_override<'a>(
    path: impl AsRef<Path>,
    commands: &'a mut Commands,
    scene_spawner: &mut SceneSpawner,
    asset_server: &AssetServer,
    manifest_override: impl FnOnce(&mut Manifest),
) -> Entity {
    let gltf_path = path.as_ref();
    let manifest_path = path.as_ref().with_extension("json");

    let mut manifest = Manifest::from_path_or_default(&manifest_path);

    manifest_override(&mut manifest);

    spawn_with_manifest(commands, scene_spawner, asset_server, gltf_path, &manifest)
}

fn spawn_with_manifest(
    commands: &mut Commands,
    scene_spawner: &mut SceneSpawner,
    asset_server: &AssetServer,
    gltf_path: &Path,
    manifest: &Manifest,
) -> Entity {
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
