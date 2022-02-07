use crate::collection;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct LocalSettingsPlugin {
    pub filename: String,
}

#[derive(Default)]
pub struct LocalSettingsLoader {
    filename: String,
    loaded: LocalSettings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Action {
    Left,
    Right,
    Special,

    Cancel,
    Confirm,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocalSettings {
    keybindings: HashMap<Action, KeyCode>,
}

impl Plugin for LocalSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LocalSettingsLoader {
            filename: self.filename.clone(),
            ..Default::default()
        })
        .add_startup_system(setup);
    }
}

impl LocalSettingsLoader {
    fn init(&mut self) {
        let file_path = dirs::config_dir()
            .unwrap()
            .join(env!("CARGO_BIN_NAME"))
            .join(&self.filename);
        match fs::try_exists(&file_path) {
            Ok(true) => {
                let f = File::open(file_path).unwrap();

                self.loaded = serde_json::from_reader(f).unwrap();
            }
            Ok(false) => {
                fs::create_dir_all(file_path.parent().unwrap()).unwrap();
                let mut f = File::create(file_path).unwrap();

                let data = LocalSettings::default();
                f.write_all(&serde_json::to_vec(&data).unwrap()).unwrap();

                self.loaded = data;
            }
            Err(err) => {
                error!("Failed to access settings file {:?}: {}", file_path, err);
            }
        }
    }

    pub fn key(&self, action: impl AsRef<Action>) -> KeyCode {
        *self.loaded.keybindings.get(action.as_ref()).unwrap()
    }
}

impl Default for LocalSettings {
    fn default() -> Self {
        Self {
            keybindings: collection! {
                Action::Right => KeyCode::A,
                Action::Left => KeyCode::E,
                Action::Special => KeyCode::Space,
                Action::Cancel => KeyCode::Escape,
                Action::Confirm => KeyCode::Return,
            },
        }
    }
}

impl AsRef<Action> for Action {
    fn as_ref(&self) -> &Action {
        self
    }
}

fn setup(mut loader: ResMut<LocalSettingsLoader>) {
    loader.init()
}
