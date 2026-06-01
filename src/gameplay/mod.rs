use avian2d::prelude::PhysicsLayer;
use bevy::prelude::*;

use crate::{AppSystems, screens::Screen};
use player::PlayerFirehoseState;

mod collisions;
mod fire;
pub mod level;
mod objects;
mod player;
mod procgen;
mod water_spray;

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub(super) enum GameLayer {
    #[default]
    Default,
    Player,
    Bubble,
}

pub const DEFAULT_HEALTH: usize = 10;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        objects::plugin,
        fire::plugin,
        player::plugin,
        water_spray::plugin,
        collisions::plugin,
    ));

    app.init_resource::<Score>();
    app.init_resource::<PlayerHealth>();

    app.add_systems(
        Update,
        (
            update_score_ui,
            update_health_ui,
            update_firehose_instruction_ui,
        )
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Score(pub usize);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerHealth(pub usize);

impl Default for PlayerHealth {
    fn default() -> Self {
        Self(DEFAULT_HEALTH)
    }
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct FirehoseInstructionText;

fn update_score_ui(score: Res<Score>, mut score_text: Single<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        let Score(score) = *score;
        score_text.0 = format!("Score: {score}");
    }
}

fn update_health_ui(
    health: Res<PlayerHealth>,
    mut health_text: Single<&mut Text, With<HealthText>>,
) {
    if health.is_changed() {
        let PlayerHealth(health) = *health;

        let whole_hearts = health / (DEFAULT_HEALTH / 5);
        let half_hearts = if health % 5 > 0 { 1 } else { 0 };
        let empty_hearts = 5 - whole_hearts - half_hearts;

        health_text.0 = std::iter::repeat_n("♥", whole_hearts)
            .chain(std::iter::repeat_n("♡", half_hearts))
            .chain(std::iter::repeat_n(" ", empty_hearts))
            .collect::<String>();
    }
}

fn update_firehose_instruction_ui(
    firehose_state: Res<State<PlayerFirehoseState>>,
    mut text: Single<&mut Text, With<FirehoseInstructionText>>,
) {
    if firehose_state.is_changed() {
        text.0 = match firehose_state.get() {
            PlayerFirehoseState::Closed => "Space: fire extinguisher".to_string(),
            PlayerFirehoseState::Open => "Space: fire extinguisher (It's Stuck!!)".to_string(),
        };
    }
}
