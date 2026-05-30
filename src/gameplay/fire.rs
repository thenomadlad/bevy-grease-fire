use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screens::Screen;

#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Fire;

pub fn spawn_fire(pos: Vec2) -> impl Bundle {
    (
        Name::new("Fire"),
        Fire,
        Sprite {
            color: Color::srgb(1.0, 0.45, 0.0),
            custom_size: Some(Vec2::splat(20.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(2.0)),
        Collider::circle(10.0),
        Sensor,
        CollisionEventsEnabled,
        DespawnOnExit(Screen::Gameplay),
    )
}
