use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{AppSystems, PausableSystems, screens::Screen};

const SPRAY_SPEED: f32 = 350.0;
const WATER_BUBBLE_JITTER: f32 = 0.5;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WaterBubble>();
    app.add_systems(
        Update,
        tick_bubble_lifetime
            .in_set(AppSystems::Computations)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct WaterBubble {
    lifetime: Timer,
}

impl Default for WaterBubble {
    fn default() -> Self {
        Self {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

pub fn spawn_water_bubble(mut commands: Commands, pos: Transform, direction: LinearVelocity) {
    let jitter = rand::rng().random_range(-WATER_BUBBLE_JITTER..WATER_BUBBLE_JITTER);
    let new_direction = Vec2::from_angle(direction.0.to_angle() + jitter) * SPRAY_SPEED;

    commands.spawn((
        WaterBubble::default(),
        Sprite {
            color: Color::srgb(0.0, 0.0, 0.99),
            custom_size: Some(Vec2::splat(12.0)),
            ..default()
        },
        pos,
        RigidBody::Dynamic,
        Collider::circle(6.0),
        CollisionEventsEnabled,
        LinearVelocity(new_direction),
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn tick_bubble_lifetime(
    mut commands: Commands,
    mut query: Query<(Entity, &mut WaterBubble)>,
    time: Res<Time>,
) {
    for (e, mut bubble) in query.iter_mut() {
        bubble.lifetime.tick(time.delta());
        if bubble.lifetime.is_finished() {
            if let Ok(mut entity) = commands.get_entity(e) {
                entity.despawn();
            }
        }
    }
}
