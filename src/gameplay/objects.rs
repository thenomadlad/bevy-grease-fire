use avian2d::prelude::*;
use bevy::prelude::*;

const WALL_T: f32 = 20.0;
const ROOM_W: f32 = 640.0;

const SHEET_COLS: u32 = 27;
const SHEET_ROWS: u32 = 18;
const TILE_PX: u32 = 16;
const TILE_PADDING: u32 = 1;

// Placeholder atlas indices (row * SHEET_COLS + col) — swap these once the sheet is visible.
const IDX_CHAIR: usize = 111;
const IDX_TABLE: usize = 7;
const IDX_KITCHEN_COUNTER: usize = 115;
const IDX_FRIDGE: usize = 416;
const IDX_STOVE: usize = 393;

macro_rules! components {
    ($($i:ident),*) => {
        $(
            #[derive(Component, Debug, Clone, Copy, Default, Reflect)]
            #[reflect(Component)]
            pub struct $i;
        )*
    };
}

components!(Stove, KitchenCounter, Fridge, Table, Chair);

pub const MAX_KNOCKABLE_HEALTH: u32 = 300;

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Knockable(pub u32);

#[derive(Resource, Clone, Reflect)]
#[reflect(Resource)]
pub struct ObjectAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for ObjectAssets {
    fn from_world(world: &mut World) -> Self {
        let texture = world
            .resource::<AssetServer>()
            .load("images/kenney_roguelike-indoors/Tilesheets/roguelikeIndoor_transparent.png");

        let layout =
            world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(TextureAtlasLayout::from_grid(
                    UVec2::new(TILE_PX, TILE_PX),
                    SHEET_COLS,
                    SHEET_ROWS,
                    Some(UVec2::new(TILE_PADDING, TILE_PADDING)),
                    None,
                ));

        Self { texture, layout }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ObjectAssets>();
    app.register_type::<Knockable>()
        .register_type::<Stove>()
        .register_type::<KitchenCounter>()
        .register_type::<Fridge>()
        .register_type::<Table>()
        .register_type::<Chair>();
}

fn atlas_sprite(assets: &ObjectAssets, index: usize, size: f32) -> Sprite {
    Sprite {
        image: assets.texture.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: assets.layout.clone(),
            index,
        }),
        custom_size: Some(Vec2::splat(size)),
        ..default()
    }
}

pub fn room_floor(room_w: f32, room_h: f32) -> impl Bundle {
    (
        Name::new("Floor"),
        Sprite {
            color: Color::srgb(0.82, 0.80, 0.76),
            custom_size: Some(Vec2::new(room_w, room_h)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    )
}

pub fn kitchen_floor(pos: Vec2, size: Vec2) -> impl Bundle {
    (
        Name::new("Kitchen Floor"),
        Sprite {
            color: Color::srgb(0.72, 0.68, 0.62),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_translation(pos.extend(0.01)),
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

pub fn stove(pos: Vec2, size: f32, assets: &ObjectAssets) -> impl Bundle {
    (
        Name::new("Stove"),
        Stove,
        atlas_sprite(assets, IDX_STOVE, size),
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Static,
        Collider::rectangle(size, size),
    )
}

pub fn kitchen_counter(pos: Vec2, size: f32, assets: &ObjectAssets) -> impl Bundle {
    (
        Name::new("Kitchen Counter"),
        KitchenCounter,
        atlas_sprite(assets, IDX_KITCHEN_COUNTER, size),
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Static,
        Collider::rectangle(size, size),
    )
}

pub fn fridge(pos: Vec2, size: f32, assets: &ObjectAssets) -> impl Bundle {
    (
        Name::new("Fridge"),
        Fridge,
        atlas_sprite(assets, IDX_FRIDGE, size),
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Static,
        Collider::rectangle(size, size),
    )
}

pub fn table(pos: Vec2, size: f32, assets: &ObjectAssets) -> impl Bundle {
    (
        Name::new("Table"),
        Table,
        Knockable(MAX_KNOCKABLE_HEALTH),
        atlas_sprite(assets, IDX_TABLE, size),
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Dynamic,
        CollisionEventsEnabled,
        Collider::rectangle(size, size),
        LockedAxes::ROTATION_LOCKED,
        LinearDamping(5.0),
    )
}

pub fn chair(pos: Vec2, size: f32, assets: &ObjectAssets) -> impl Bundle {
    (
        Name::new("Chair"),
        Chair,
        Knockable(MAX_KNOCKABLE_HEALTH),
        atlas_sprite(assets, IDX_CHAIR, size),
        Transform::from_translation(pos.extend(1.0)),
        RigidBody::Dynamic,
        CollisionEventsEnabled,
        Collider::rectangle(size, size),
        LockedAxes::ROTATION_LOCKED,
        LinearDamping(5.0),
    )
}
