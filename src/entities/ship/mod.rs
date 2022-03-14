use crate::utils::local_settings::{Action, LocalSettingsLoader};
use crate::utils::spawn::spawn_model_as_child;
use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
use input_state::{PlayerShipInputState, TurnDirection};

mod input_state;

const TERMINAL_VELOCITY_X: f32 = 100.0;
const TERMINAL_VELOCITY_Z: &[f32] = &[120.0, 240.0, 350.0, 420.0, 500.0, 560.0, 600.0];

const DAMPING_X: f32 = 150.0;

const FORCE_X: f32 = 2000.0;
const FORCE_Z: f32 = 10000.0;

const LABEL_HANDLE_USER_INPUT: &str = "a3abb244-887a-469d-8a34-f6c154b0d310";
pub const LABEL_FLY_SHIP: &str = "af0a465f-99e8-4023-bcc1-921ff9a1e00a";

pub struct ShipControlPlugin;

#[derive(Component)]
pub struct PlayerShipMarker;

#[derive(Default)]
pub struct PlayerShipDescriptor {
    pub speed_level: usize,
}

impl Plugin for ShipControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerShipInputState>()
            .init_resource::<PlayerShipDescriptor>()
            .add_startup_system(spawn_player_ship)
            .add_system(handle_user_input.label(LABEL_HANDLE_USER_INPUT))
            .add_system(
                fly_ship
                    .label(LABEL_FLY_SHIP)
                    .after(LABEL_HANDLE_USER_INPUT),
            );
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
                linear_damping: 20.0,
                ..Default::default()
            }
            .into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
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
    time: Res<Time>,
    mut q_spaceship: Query<(&mut RigidBodyVelocityComponent,), With<PlayerShipMarker>>,
    mut descriptor: ResMut<PlayerShipDescriptor>,
    state: Res<PlayerShipInputState>,
) {
    let Ok((mut vel, )) = q_spaceship.get_single_mut() else {
        return
    };

    if state.special {
        descriptor.speed_level += 1;
    }

    let dt = time.delta().as_secs_f32();

    vel.linvel.x = vel.linvel.x - vel.linvel.x.signum() * DAMPING_X * dt;

    let target_x_force = match state.turning {
        TurnDirection::Left => -FORCE_X,
        TurnDirection::Right => FORCE_X,
        TurnDirection::None => 0.0,
    };

    vel.linvel.x =
        (vel.linvel.x + target_x_force * dt).clamp(-TERMINAL_VELOCITY_X, TERMINAL_VELOCITY_X);
    if vel.linvel.x.abs() < 5.0 {
        vel.linvel.x = 0.0;
    }
    vel.linvel.z += FORCE_Z * dt;
    let terminal_velocity_z =
        TERMINAL_VELOCITY_Z[descriptor.speed_level.min(TERMINAL_VELOCITY_Z.len() - 1)];
    vel.linvel.z = vel
        .linvel
        .z
        .clamp(-terminal_velocity_z, terminal_velocity_z);
}

pub fn handle_user_input(
    local_settings: Res<LocalSettingsLoader>,
    input: Res<Input<KeyCode>>,
    mut ship_state: ResMut<PlayerShipInputState>,
) {
    if input.pressed(local_settings.key(Action::Left)) {
        ship_state.turning = TurnDirection::Left;
    } else if input.pressed(local_settings.key(Action::Right)) {
        ship_state.turning = TurnDirection::Right;
    } else {
        ship_state.turning = TurnDirection::None;
    }

    ship_state.special = false;
    if input.just_pressed(local_settings.key(Action::Special)) {
        ship_state.special = true;
    }
}
