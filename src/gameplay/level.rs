use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    gameplay::{
        FirehoseInstructionText, HealthText, ScoreText,
        fire::{FireAssets, spawn_fire},
        objects::{
            ObjectAssets, chair, fridge, kitchen_counter, kitchen_floor, room_floor, stove, table,
            wall_east, wall_north, wall_south, wall_west,
        },
        player::{CharacterAssets, player},
        procgen::{NoiseScatterGenerator, ObjectKind, RoomGenerator},
    },
    screens::Screen,
};

pub const CELL_SIZE: f32 = 32.0;

const ROOM_BOUNDS: Rect = Rect {
    min: Vec2::new(-290.0, -240.0),
    max: Vec2::new(290.0, 240.0),
};

fn cell_center(row: usize, col: usize) -> Vec2 {
    Vec2::new(
        ROOM_BOUNDS.min.x + row as f32 * CELL_SIZE + CELL_SIZE / 2.0,
        ROOM_BOUNDS.min.y + col as f32 * CELL_SIZE + CELL_SIZE / 2.0,
    )
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    object_assets: Res<ObjectAssets>,
    character_assets: Res<CharacterAssets>,
    fire_assets: Res<FireAssets>,
) {
    let kitchen_zone = Rect::from_corners(
        ROOM_BOUNDS.min,
        ROOM_BOUNDS.min + Vec2::splat(3.0 * CELL_SIZE),
    );

    let stove_pos = cell_center(0, 0);
    let counter_pos = cell_center(1, 0);
    let fridge_pos = cell_center(2, 0);
    let player_pos = cell_center(1, 2);

    let level = commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
            children![
                (
                    Name::new("Gameplay Music"),
                    music(level_assets.music.clone())
                ),
                room_floor(ROOM_BOUNDS.width(), ROOM_BOUNDS.height()),
                kitchen_floor(
                    ROOM_BOUNDS.min + Vec2::splat(1.5 * CELL_SIZE),
                    Vec2::splat(3.0 * CELL_SIZE),
                ),
                wall_north(),
                wall_south(),
                wall_east(),
                wall_west(),
                player(player_pos, &character_assets),
            ],
        ))
        .id();

    // Hardcoded kitchen corner
    for entity in [
        commands
            .spawn(stove(stove_pos, CELL_SIZE, &object_assets))
            .id(),
        commands
            .spawn(kitchen_counter(counter_pos, CELL_SIZE, &object_assets))
            .id(),
        commands
            .spawn(fridge(fridge_pos, CELL_SIZE, &object_assets))
            .id(),
    ] {
        commands.entity(level).add_child(entity);
    }

    // Procedurally generated furniture
    let generator = NoiseScatterGenerator {
        density: 0.2,
        exclusion_rects: vec![kitchen_zone],
    };
    for obj in generator.generate(ROOM_BOUNDS, 42, CELL_SIZE) {
        let child = match obj.kind {
            ObjectKind::Table => commands
                .spawn(table(obj.position, CELL_SIZE, &object_assets))
                .id(),
            ObjectKind::Chair => commands
                .spawn(chair(obj.position, CELL_SIZE, &object_assets))
                .id(),
            ObjectKind::KitchenCounter => commands
                .spawn(kitchen_counter(obj.position, CELL_SIZE, &object_assets))
                .id(),
            ObjectKind::Fridge => commands
                .spawn(fridge(obj.position, CELL_SIZE, &object_assets))
                .id(),
            ObjectKind::Stove => commands
                .spawn(stove(obj.position, CELL_SIZE, &object_assets))
                .id(),
        };
        commands.entity(level).add_child(child);
    }

    commands.spawn(spawn_fire(stove_pos, &fire_assets));

    commands.spawn((
        Name::new("Score HUD"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(16.0),
            ..default()
        },
        DespawnOnExit(Screen::Gameplay),
        children![(
            Name::new("Score Text"),
            ScoreText,
            Text("Score: 0".to_string()),
            TextFont::from_font_size(24.0),
            TextColor(Color::WHITE),
        )],
    ));

    commands.spawn((
        Name::new("Health HUD"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(16.0),
            ..default()
        },
        DespawnOnExit(Screen::Gameplay),
        children![(
            Name::new("Health Text"),
            HealthText,
            Text("♥♥♥♥♥".to_string()),
            TextFont::from_font_size(24.0),
            TextColor(Color::srgb(1.0, 0.2, 0.2)),
        )],
    ));

    commands.spawn((
        Name::new("Move Instructions"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(16.0),
            ..default()
        },
        DespawnOnExit(Screen::Gameplay),
        children![(
            Name::new("Move Instructions Text"),
            Text("WASD / Arrows to move".to_string()),
            TextFont::from_font_size(16.0),
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        )],
    ));

    commands.spawn((
        Name::new("Firehose Instructions"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            right: Val::Px(16.0),
            ..default()
        },
        DespawnOnExit(Screen::Gameplay),
        children![(
            Name::new("Firehose Instructions Text"),
            FirehoseInstructionText,
            Text("Space: fire extinguisher".to_string()),
            TextFont::from_font_size(16.0),
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        )],
    ));
}
