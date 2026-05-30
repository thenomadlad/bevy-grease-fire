use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{AppSystems, PausableSystems, gameplay::water_spray::spawn_water_bubble};

const PLAYER_JITTER: f32 = 0.5;
const MIN_SPEED: f32 = 80.0;
const MAX_SPEED: f32 = 600.0;
const IDLE_RECOIL_FACTOR: f32 = 0.25;
// speed doubles roughly every 0.35s; reaches MAX_SPEED in ~1s
const SPEED_GROWTH_RATE: f32 = 2.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum PlayerFirehoseState {
    #[default]
    Closed,
    Open,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.init_state::<PlayerFirehoseState>();
    app.add_systems(
        Update,
        (record_player_directional_input, handle_firehose_state)
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );

    app.add_systems(
        Update,
        spray_water
            .run_if(in_state(PlayerFirehoseState::Open))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

pub fn player(pos: Vec2) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Sprite {
            color: Color::srgb(0.2, 0.4, 0.85),
            custom_size: Some(Vec2::splat(24.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(2.0)),
        RigidBody::Dynamic,
        Collider::rectangle(24.0, 24.0),
        LockedAxes::ROTATION_LOCKED,
    )
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    firehose_state: Res<State<PlayerFirehoseState>>,
    player_velocity: Single<&mut LinearVelocity, With<Player>>,
    mut last_hose_dir: Local<Vec2>,
    mut hold_secs: Local<f32>,
    time: Res<Time>,
) {
    if last_hose_dir.length_squared() == 0.0 {
        *last_hose_dir = Vec2::Y;
    }

    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }
    let intent = intent.normalize_or_zero();

    if intent.length_squared() > 0.0 {
        *hold_secs += time.delta_secs();
    } else {
        *hold_secs = 0.0;
    }

    let speed = (MIN_SPEED * (*hold_secs * SPEED_GROWTH_RATE).exp()).min(MAX_SPEED);

    let new_velocity = match firehose_state.get() {
        PlayerFirehoseState::Closed => intent * speed,
        PlayerFirehoseState::Open => {
            let jitter = rand::rng().random_range(-PLAYER_JITTER..PLAYER_JITTER);
            if intent.length_squared() > 0.0 {
                *last_hose_dir = intent;
                Vec2::from_angle((-intent).to_angle() + jitter) * speed
            } else {
                Vec2::from_angle((-*last_hose_dir).to_angle() + jitter)
                    * MAX_SPEED
                    * IDLE_RECOIL_FACTOR
            }
        }
    };

    let mut velocity = player_velocity.into_inner();
    *velocity = LinearVelocity(new_velocity);
}

fn handle_firehose_state(
    input: Res<ButtonInput<KeyCode>>,
    mut next_firehose_state: ResMut<NextState<PlayerFirehoseState>>,
) {
    if input.pressed(KeyCode::Space) {
        next_firehose_state.set(PlayerFirehoseState::Open);
    }
}

fn spray_water(
    commands: Commands,
    player: Single<(&Transform, &LinearVelocity), With<Player>>,
    mut last_hose_dir: Local<Vec2>,
    mut spray_timer: Local<Option<Timer>>,
    time: Res<Time>,
) {
    if last_hose_dir.length_squared() == 0.0 {
        *last_hose_dir = Vec2::Y;
    }

    let (pos, velocity) = player.into_inner();

    // When velocity exceeds idle recoil magnitude, the player is pressing keys.
    // Water sprays opposite to their kickback direction.
    if velocity.0.length() > MAX_SPEED * IDLE_RECOIL_FACTOR * 1.5 {
        *last_hose_dir = (-velocity.0).normalize();
    }

    let timer =
        spray_timer.get_or_insert_with(|| Timer::from_seconds(1.0 / 40.0, TimerMode::Repeating));
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    let mut spawn_transform = *pos;
    spawn_transform.translation += last_hose_dir.extend(0.0) * 40.0;
    spawn_water_bubble(commands, spawn_transform, LinearVelocity(*last_hose_dir));
}
