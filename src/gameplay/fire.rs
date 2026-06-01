use avian2d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use rand::Rng;

use crate::{AppSystems, PausableSystems, screens::Screen};

const FLAME_COLORS: &[Color] = &[
    Color::srgba(0.90, 0.25, 0.00, 0.95),
    Color::srgba(1.00, 0.35, 0.00, 0.95),
    Color::srgba(1.00, 0.50, 0.00, 0.90),
    Color::srgba(1.00, 0.65, 0.10, 0.85),
    Color::srgba(1.00, 0.80, 0.20, 0.80),
];

#[derive(Resource)]
pub(crate) struct FireAssets {
    mesh: Handle<Mesh>,
    materials: Vec<Handle<ColorMaterial>>,
}

impl FromWorld for FireAssets {
    fn from_world(world: &mut World) -> Self {
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(build_flame_mesh());
        let mut mat_assets = world.resource_mut::<Assets<ColorMaterial>>();
        let materials = FLAME_COLORS
            .iter()
            .map(|&c| mat_assets.add(ColorMaterial::from_color(c)))
            .collect();
        Self { mesh, materials }
    }
}

// Tip at top (+Y), round base at bottom (−Y). Fan-triangulated from center.
fn build_flame_mesh() -> Mesh {
    let outline: &[(f32, f32)] = &[
        (0.00, 1.00),
        (0.40, 0.60),
        (0.70, 0.10),
        (0.50, -0.50),
        (0.25, -0.85),
        (0.00, -0.95),
        (-0.25, -0.85),
        (-0.50, -0.50),
        (-0.70, 0.10),
        (-0.40, 0.60),
    ];
    let scale = 13.0_f32;
    let n = outline.len() as u32;

    let mut positions: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]];
    positions.extend(outline.iter().map(|&(x, y)| [x * scale, y * scale, 0.0]));

    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; positions.len()];

    let mut uvs: Vec<[f32; 2]> = vec![[0.5, 0.5]];
    uvs.extend(
        outline
            .iter()
            .map(|&(x, y)| [(x + 1.0) * 0.5, 1.0 - (y + 1.0) * 0.5]),
    );

    let mut indices: Vec<u32> = Vec::new();
    for i in 1..=n {
        indices.push(0);
        indices.push(i);
        indices.push(if i == n { 1 } else { i + 1 });
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}

#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Fire;

#[derive(Component)]
struct FireFlicker {
    timer: Timer,
}

pub fn spawn_fire(pos: Vec2, assets: &FireAssets) -> impl Bundle {
    let mat_idx = rand::rng().random_range(0..assets.materials.len());
    (
        Name::new("Fire"),
        Fire,
        Mesh2d(assets.mesh.clone()),
        MeshMaterial2d(assets.materials[mat_idx].clone()),
        Transform::from_translation(pos.extend(2.0)),
        FireFlicker {
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
        },
        Collider::circle(10.0),
        Sensor,
        CollisionEventsEnabled,
        DespawnOnExit(Screen::Gameplay),
    )
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Fire>();
    app.init_resource::<FireAssets>();
    app.add_systems(
        Update,
        flicker_fire
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

fn flicker_fire(
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>, &mut FireFlicker)>,
    assets: Res<FireAssets>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();
    for (mut material, mut flicker) in query.iter_mut() {
        flicker.timer.tick(time.delta());
        if flicker.timer.just_finished() {
            let idx = rng.random_range(0..assets.materials.len());
            material.0 = assets.materials[idx].clone();
        }
    }
}
