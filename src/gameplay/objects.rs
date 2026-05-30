use avian2d::prelude::*;
use bevy::prelude::*;

// Interior playable area: 600×500. Walls are 20px thick on each side.
const WALL_T: f32 = 20.0;
const ROOM_W: f32 = 640.0;
const ROOM_H: f32 = 540.0;

macro_rules! components {
    ($($i:ident),*) => {
        $(
            #[derive(Component, Debug, Clone, Copy, Default, Reflect)]
            #[reflect(Component)]
            pub struct $i;
        )*
    };
}

components!(Knocked, Stove, KitchenCounter, Fridge, Table, Chair);

pub const MAX_KNOCKABLE_HEALTH: u32 = 300;

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Knockable(pub u32);

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Knocked>()
        .register_type::<Knockable>()
        .register_type::<Stove>()
        .register_type::<KitchenCounter>()
        .register_type::<Fridge>()
        .register_type::<Table>()
        .register_type::<Chair>();
}

pub fn room_floor() -> impl Bundle {
    (
        Name::new("Floor"),
        Sprite {
            color: Color::srgb(0.18, 0.16, 0.13),
            custom_size: Some(Vec2::new(ROOM_W, ROOM_H)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    )
}

pub fn wall_north() -> impl Bundle {
    static_wall(
        "Wall North",
        Vec2::new(0.0, 260.0),
        Vec2::new(ROOM_W, WALL_T),
    )
}

pub fn wall_south() -> impl Bundle {
    static_wall(
        "Wall South",
        Vec2::new(0.0, -260.0),
        Vec2::new(ROOM_W, WALL_T),
    )
}

pub fn wall_east() -> impl Bundle {
    static_wall("Wall East", Vec2::new(310.0, 0.0), Vec2::new(WALL_T, 500.0))
}

pub fn wall_west() -> impl Bundle {
    static_wall(
        "Wall West",
        Vec2::new(-310.0, 0.0),
        Vec2::new(WALL_T, 500.0),
    )
}

fn static_wall(name: &'static str, pos: Vec2, size: Vec2) -> impl Bundle {
    (
        Name::new(name),
        Sprite {
            color: Color::srgb(0.45, 0.45, 0.48),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    )
}

pub fn stove(pos: Vec2) -> impl Bundle {
    (
        Name::new("Stove"),
        Stove,
        Sprite {
            color: Color::srgb(0.12, 0.08, 0.08),
            custom_size: Some(Vec2::new(80.0, 60.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Static,
        Collider::rectangle(80.0, 60.0),
    )
}

pub fn kitchen_counter(pos: Vec2) -> impl Bundle {
    (
        Name::new("Kitchen Counter"),
        KitchenCounter,
        Sprite {
            color: Color::srgb(0.58, 0.55, 0.48),
            custom_size: Some(Vec2::new(200.0, 40.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Static,
        Collider::rectangle(200.0, 40.0),
    )
}

pub fn fridge(pos: Vec2) -> impl Bundle {
    (
        Name::new("Fridge"),
        Fridge,
        Sprite {
            color: Color::srgb(0.88, 0.90, 0.92),
            custom_size: Some(Vec2::new(50.0, 80.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 80.0),
    )
}

pub fn table(pos: Vec2) -> impl Bundle {
    (
        Name::new("Table"),
        Table,
        Knockable(MAX_KNOCKABLE_HEALTH),
        Sprite {
            color: Color::srgb(0.45, 0.28, 0.12),
            custom_size: Some(Vec2::new(120.0, 80.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Dynamic,
        CollisionEventsEnabled,
        Collider::rectangle(120.0, 80.0),
        LockedAxes::ROTATION_LOCKED,
        LinearDamping(5.0),
    )
}

pub fn chair(pos: Vec2) -> impl Bundle {
    (
        Name::new("Chair"),
        Chair,
        Knockable(MAX_KNOCKABLE_HEALTH),
        Sprite {
            color: Color::srgb(0.32, 0.20, 0.08),
            custom_size: Some(Vec2::new(36.0, 36.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Dynamic,
        CollisionEventsEnabled,
        Collider::rectangle(36.0, 36.0),
        LockedAxes::ROTATION_LOCKED,
        LinearDamping(5.0),
    )
}
