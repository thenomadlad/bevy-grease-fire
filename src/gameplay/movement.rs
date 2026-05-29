use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        apply_movement
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// Direction the entity wants to move, normalized.
    pub player_direction: Vec2,
    /// Maximum speed in pixels per second.
    pub max_speed: f32,
    /// Random angular deviation added to movement each frame, in radians.
    pub jitter: f32,
    /// movement will be predicated on the hose running also
    pub hose_direction: Vec2,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            player_direction: Vec2::ZERO,
            max_speed: 600.0,
            jitter: 0.5,
            hose_direction: Vec2::new(0.0, 1.0),
        }
    }
}

fn apply_movement(query: Single<(&MovementController, &mut LinearVelocity)>) {
    let (controller, mut velocity) = query.into_inner();
    if controller.player_direction == Vec2::ZERO {
        *velocity = LinearVelocity::ZERO;
        return;
    }
    let jitter_angle = rand::rng().random_range(-controller.jitter..controller.jitter);
    let jittered = Vec2::from_angle(controller.player_direction.to_angle() + jitter_angle);
    *velocity = LinearVelocity(jittered * controller.max_speed);
}
