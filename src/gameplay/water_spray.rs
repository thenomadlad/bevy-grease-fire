use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{AppSystems, PausableSystems, gameplay::GameLayer, screens::Screen};

const SPRAY_SPEED: f32 = 350.0;
const WATER_BUBBLE_JITTER: f32 = 0.5;

const BUBBLE_COLORS: &[Color] = &[
    Color::srgba(0.50, 0.82, 1.00, 0.80),
    Color::srgba(0.40, 0.75, 0.96, 0.75),
    Color::srgba(0.62, 0.88, 1.00, 0.85),
    Color::srgba(0.45, 0.79, 0.98, 0.78),
    Color::srgba(0.55, 0.84, 1.00, 0.82),
    Color::srgba(0.68, 0.90, 1.00, 0.80),
];

#[derive(Resource)]
pub(crate) struct WaterBubbleAssets {
    mesh: Handle<Mesh>,
    materials: Vec<Handle<ColorMaterial>>,
}

impl FromWorld for WaterBubbleAssets {
    fn from_world(world: &mut World) -> Self {
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Circle::new(6.0));
        let mut mat_assets = world.resource_mut::<Assets<ColorMaterial>>();
        let materials = BUBBLE_COLORS
            .iter()
            .map(|&c| mat_assets.add(ColorMaterial::from_color(c)))
            .collect();
        Self { mesh, materials }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WaterBubble>();
    app.init_resource::<WaterBubbleAssets>();
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

pub fn spawn_water_bubble(
    mut commands: Commands,
    assets: &WaterBubbleAssets,
    pos: Transform,
    direction: LinearVelocity,
) {
    let jitter = rand::rng().random_range(-WATER_BUBBLE_JITTER..WATER_BUBBLE_JITTER);
    let new_direction = Vec2::from_angle(direction.0.to_angle() + jitter) * SPRAY_SPEED;
    let mat_idx = rand::rng().random_range(0..assets.materials.len());

    commands.spawn((
        WaterBubble::default(),
        Mesh2d(assets.mesh.clone()),
        MeshMaterial2d(assets.materials[mat_idx].clone()),
        pos,
        RigidBody::Dynamic,
        Collider::circle(6.0),
        CollisionEventsEnabled,
        CollisionLayers::new(GameLayer::Bubble, [GameLayer::Default]),
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
        if bubble.lifetime.is_finished()
            && let Ok(mut entity) = commands.get_entity(e)
        {
            entity.despawn();
        }
    }
}
