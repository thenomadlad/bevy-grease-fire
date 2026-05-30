use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    gameplay::{
        HealthText, ScoreText,
        fire::spawn_fire,
        objects::{
            chair, fridge, kitchen_counter, room_floor, stove, table, wall_east, wall_north,
            wall_south, wall_west,
        },
        player::player,
    },
    screens::Screen,
};

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

pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            ),
            room_floor(),
            wall_north(),
            wall_south(),
            wall_east(),
            wall_west(),
            stove(Vec2::new(-220.0, 215.0)),
            kitchen_counter(Vec2::new(-90.0, 220.0)),
            fridge(Vec2::new(260.0, 80.0)),
            table(Vec2::new(30.0, 0.0)),
            chair(Vec2::new(-40.0, -90.0)),
            chair(Vec2::new(100.0, -90.0)),
            player(Vec2::new(0.0, -170.0)),
        ],
    ));

    commands.spawn(spawn_fire(Vec2::new(-220.0, 215.0)));

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
}
