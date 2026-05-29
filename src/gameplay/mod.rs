use bevy::prelude::*;

pub mod level;
mod movement;
mod objects;
mod player;
mod water_spray;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        movement::plugin,
        objects::plugin,
        player::plugin,
        water_spray::plugin,
    ));
}
