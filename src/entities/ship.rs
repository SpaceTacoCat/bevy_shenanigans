use crate::utils::local_settings::{Action, LocalSettingsLoader};
use crate::MainCameraMarker;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct ShipAndControlPlugin;

const TERMINAL_VELOCITY_X: f32 = 12.0;
const TERMINAL_VELOCITY_Z: f32 = 12.0;

const FORCE_X: f32 = 1000.0;

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
}

impl Plugin for ShipAndControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerShipState>()
            .add_startup_system(spawn_player_ship)
            .add_system(control_spaceship.label("control"))
            .add_system(fly_ship.label("fly").after("control"))
            .add_system(lock_y_position.after("fly"))
            .add_system(camera_follow_spaceship.after("fly"));
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
    let entity = commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            damping: RigidBodyDamping {
                linear_damping: 10.0,
                ..Default::default()
            }.into(),
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
        .insert(PlayerShipMarker)
        .id();

    scene_spawner.spawn_as_child(asset_server.load("models/spaceship.gltf#Scene0"), entity);
}

pub fn camera_follow_spaceship(
    mut q_camera: Query<&mut Transform, With<MainCameraMarker>>,
    q_spaceship: Query<&Children, With<PlayerShipMarker>>,
    q_transforms: Query<&Transform, Without<MainCameraMarker>>,
) {
    let mut camera = if let Ok(camera) = q_camera.get_single_mut() {
        camera
    } else {
        return;
    };
    let spaceship = if let Ok(children) = q_spaceship.get_single() {
        children.first().unwrap()
    } else {
        return;
    };

    let ship_object = q_transforms.get(*spaceship).unwrap();

    camera.translation = ship_object.translation + Vec3::new(0.0, 30.0, -40.0);
    camera.look_at(ship_object.translation + Vec3::Y * 10.0, Vec3::Y);
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
}

pub fn lock_y_position(
    mut q_spaceship: Query<(&Transform, &mut RigidBodyVelocityComponent), With<PlayerShipMarker>>,
) {
    let Ok((t, mut vel)) = q_spaceship.get_single_mut() else {
        return
    };

    if t.translation.y < 0.0 {
        vel.linvel.y = 0.0;
    }
}

pub fn control_spaceship(
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
}
