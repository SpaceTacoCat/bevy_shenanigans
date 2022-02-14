use crate::utils::local_settings::{Action, LocalSettingsLoader};
use crate::utils::spawn::spawn_model_as_child;
use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

pub struct ShipControlPlugin;

const TERMINAL_VELOCITY_X: f32 = 100.0;
const TERMINAL_VELOCITY_Z: f32 = 12.0;

const FORCE_X: f32 = 2000.0;
const FORCE_Z: f32 = 10000.0;

const SHIP_MINIMUM_Y_POS: f32 = 0.0;

#[derive(Component)]
pub struct PlayerShipMarker;

pub enum TurnDirection {
    Left = -1,
    Right = 1,
    None = 0,
}

#[derive(Default)]
pub struct PlayerShipState {
    turning: TurnDirection,
    special: bool,
}

const LABEL_HANDLE_USER_INPUT: &str = "control";
pub const LABEL_FLY_SHIP: &str = "fly";

impl Plugin for ShipControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerShipState>()
            .add_startup_system(spawn_player_ship)
            .add_system(handle_user_input.label(LABEL_HANDLE_USER_INPUT))
            .add_system(
                fly_ship
                    .label(LABEL_FLY_SHIP)
                    .after(LABEL_HANDLE_USER_INPUT),
            )
            .add_system(lock_y_position.after(LABEL_FLY_SHIP));
    }
}

impl Default for TurnDirection {
    fn default() -> Self {
        TurnDirection::None
    }
}

pub fn spawn_player_ship(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
) {
    let mut entity_commands = commands.spawn();

    entity_commands
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            damping: RigidBodyDamping {
                linear_damping: 8.0,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1.0, 1.0, 1.0).into(),
            ..Default::default()
        })
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default())
        .insert(PlayerShipMarker);

    spawn_model_as_child(
        "models/spaceship.gltf",
        &mut scene_spawner,
        &asset_server,
        &mut entity_commands,
    );
}

pub fn fly_ship(
    mut q_spaceship: Query<
        (
            &mut RigidBodyVelocityComponent,
            &mut RigidBodyForcesComponent,
        ),
        With<PlayerShipMarker>,
    >,
    state: Res<PlayerShipState>,
) {
    let Ok((mut vel, mut forces)) = q_spaceship.get_single_mut() else {
        return
    };

    let target_x_force = match state.turning {
        TurnDirection::Left => -FORCE_X,
        TurnDirection::Right => FORCE_X,
        TurnDirection::None => 0.0,
    };

    forces.force.x = target_x_force;
    vel.linvel.x = vel.linvel.x.abs().min(TERMINAL_VELOCITY_X) * vel.linvel.x.signum();

    forces.force.z = if state.special { FORCE_Z } else { 0.0 };
}

/// TODO: This probably needs updating
pub fn lock_y_position(
    mut q_spaceship: Query<(&Transform, &mut RigidBodyVelocityComponent), With<PlayerShipMarker>>,
) {
    let Ok((t, mut vel)) = q_spaceship.get_single_mut() else {
        return
    };

    if t.translation.y < SHIP_MINIMUM_Y_POS {
        vel.linvel.y = 0.0;
    }
}

pub fn handle_user_input(
    local_settings: Res<LocalSettingsLoader>,
    input: Res<Input<KeyCode>>,
    mut ship_state: ResMut<PlayerShipState>,
) {
    if input.pressed(local_settings.key(Action::Left)) {
        ship_state.turning = TurnDirection::Left;
    } else if input.pressed(local_settings.key(Action::Right)) {
        ship_state.turning = TurnDirection::Right;
    } else {
        ship_state.turning = TurnDirection::None;
    }

    if input.pressed(local_settings.key(Action::Special)) {
        // TODO: In normal game make this `just_pressed`
        ship_state.special = true;
    } else {
        ship_state.special = false;
    }
}
