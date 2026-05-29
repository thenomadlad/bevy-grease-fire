use std::collections::HashSet;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, gameplay::player::Player, screens::Screen};

const SPRAY_SPEED: f32 = 350.0;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WaterBubble>();
    app.add_systems(
        Update,
        (tick_bubble_lifetime, despawn_bubble_on_collision)
            .in_set(AppSystems::Update)
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
        LinearVelocity(direction.0.normalize_or_zero() * SPRAY_SPEED),
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

fn despawn_bubble_on_collision(
    mut commands: Commands,
    mut collision_reader: MessageReader<CollisionStart>,
    bubble_query: Query<(), With<WaterBubble>>,
    player_query: Query<(), With<Player>>,
) {
    let mut to_despawn: HashSet<Entity> = HashSet::new();
    for event in collision_reader.read() {
        for (bubble_e, other_e) in [
            (event.collider1, event.collider2),
            (event.collider2, event.collider1),
        ] {
            if bubble_query.contains(bubble_e)
                && !player_query.contains(other_e)
                && !bubble_query.contains(other_e)
            {
                to_despawn.insert(bubble_e);
            }
        }
    }
    for e in to_despawn {
        if let Ok(mut entity) = commands.get_entity(e) {
            entity.despawn();
        }
    }
}
