use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    gameplay::{movement::MovementController, water_spray::spawn_water_bubble},
};

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
        MovementController::default(),
    )
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    firehose_state: Res<State<PlayerFirehoseState>>,
    mut controller_query: Single<&mut MovementController, With<Player>>,
) {
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
        // player moves backwards while water is sprayed
        controller_query.player_direction = intent
            * match firehose_state.get() {
                PlayerFirehoseState::Closed => 1.0,
                PlayerFirehoseState::Open => -1.0,
            };

        controller_query.hose_direction = intent;
    } else {
        // player gets kickback of water spray
        controller_query.player_direction = intent
            * match firehose_state.get() {
                PlayerFirehoseState::Closed => 1.0,
                PlayerFirehoseState::Open => -1.0,
            };
    }
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
    items: Single<(&MovementController, &Transform), With<Player>>,
    mut spray_timer: Local<Option<Timer>>,
    time: Res<Time>,
) {
    let timer =
        spray_timer.get_or_insert_with(|| Timer::from_seconds(1.0 / 40.0, TimerMode::Repeating));
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    let (movement, pos) = items.into_inner();
    let spray_direction = movement.hose_direction;
    let velocity = LinearVelocity(spray_direction);
    let mut spawn_transform = *pos;

    spawn_transform.translation += spray_direction.extend(0.0) * 20.0;
    spawn_water_bubble(commands, spawn_transform, velocity);
}
