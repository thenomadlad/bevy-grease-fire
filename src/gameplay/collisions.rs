use std::collections::HashSet;

use avian2d::collision::collision_events::{CollisionEnd, CollisionStart};
use bevy::prelude::*;
use rand::Rng;

const PLAYER_KNOCKABLE_DAMAGE: u32 = 60;
const BUBBLE_KNOCKABLE_DAMAGE: u32 = 8;
const FIRE_SPAWN_DIST: f32 = 30.0;
const BURN_DAMAGE_INTERVAL_SECS: f32 = 0.5;

use crate::{
    AppSystems,
    gameplay::{
        PlayerHealth, Score,
        fire::{Fire, spawn_fire},
        objects::{Knockable, Knocked, MAX_KNOCKABLE_HEALTH},
        player::Player,
        water_spray::WaterBubble,
    },
    screens::Screen,
};

#[derive(Resource, Default)]
struct BurnContacts(HashSet<Entity>);

#[derive(Resource)]
struct BurnDamageTimer(Timer);

impl Default for BurnDamageTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(BURN_DAMAGE_INTERVAL_SECS, TimerMode::Repeating))
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BurnContacts>();
    app.init_resource::<BurnDamageTimer>();

    app.add_systems(OnEnter(Screen::Gameplay), reset_burn_state);

    app.add_systems(Update, handle_collisions.in_set(AppSystems::Cleanups));
    app.add_systems(
        Update,
        (track_burn_end, apply_burn_damage).in_set(AppSystems::Cleanups),
    );
}

fn reset_burn_state(mut contacts: ResMut<BurnContacts>, mut timer: ResMut<BurnDamageTimer>) {
    contacts.0.clear();
    timer.0.reset();
}

fn handle_collisions(
    mut commands: Commands,
    mut collision_reader: MessageReader<CollisionStart>,
    bubble_query: Query<(), With<WaterBubble>>,
    fire_query: Query<(), With<Fire>>,
    player: Single<Entity, With<Player>>,
    knockable_query: Query<(), (With<Knockable>, Without<Knocked>)>,
    knockable_transform: Query<&Transform, With<Knockable>>,
    mut score: ResMut<Score>,
    mut burn_contacts: ResMut<BurnContacts>,
) {
    let mut to_despawn: HashSet<Entity> = HashSet::new();

    for event in collision_reader.read() {
        for (lhs, rhs) in [
            (event.collider1, event.collider2),
            (event.collider2, event.collider1),
        ] {
            let is_lhs_bubble = bubble_query.contains(lhs);
            let is_lhs_player = *player == lhs;
            let is_rhs_fire = fire_query.contains(rhs);
            let is_rhs_knockable = knockable_query.contains(rhs);

            if is_lhs_bubble {
                to_despawn.insert(lhs);

                if is_rhs_fire {
                    info!("collision: bubble {:?} hit fire {:?} → extinguished", lhs, rhs);
                    to_despawn.insert(rhs);
                    score.0 += 1;
                }
            }

            if is_lhs_player && is_rhs_fire {
                info!("collision: player hit fire {:?} → burn started", rhs);
                burn_contacts.0.insert(rhs);
            }

            if is_rhs_knockable {
                if let Ok(transform) = knockable_transform.get(rhs) {
                    if is_lhs_player {
                        info!("collision: player hit knockable {:?} → rolling fire spawn (p={PLAYER_KNOCKED_FIRE_HAZARD_RATE})", rhs);
                        knocked_off_fire(
                            PLAYER_KNOCKED_FIRE_HAZARD_RATE,
                            3,
                            transform,
                            &mut commands,
                        );
                    } else if is_lhs_bubble {
                        info!("collision: bubble hit knockable {:?} → rolling fire spawn (p={BUBBLE_KNOCKED_FIRE_HAZARD_RATE})", rhs);
                        knocked_off_fire(
                            BUBBLE_KNOCKED_FIRE_HAZARD_RATE,
                            1,
                            transform,
                            &mut commands,
                        );
                    }
                }

                commands.entity(rhs).remove::<Knockable>().insert(Knocked);
            }
        }
    }

    for e in to_despawn {
        if let Ok(mut entity) = commands.get_entity(e) {
            entity.despawn();
        }
    }
}

fn track_burn_end(
    mut collision_reader: MessageReader<CollisionEnd>,
    player: Single<Entity, With<Player>>,
    fire_query: Query<(), With<Fire>>,
    mut burn_contacts: ResMut<BurnContacts>,
) {
    for event in collision_reader.read() {
        for (lhs, rhs) in [
            (event.collider1, event.collider2),
            (event.collider2, event.collider1),
        ] {
            if lhs == *player && fire_query.contains(rhs) {
                burn_contacts.0.remove(&rhs);
            }
        }
    }
}

fn apply_burn_damage(
    time: Res<Time>,
    mut burn_contacts: ResMut<BurnContacts>,
    mut timer: ResMut<BurnDamageTimer>,
    fire_query: Query<(), With<Fire>>,
    mut health: ResMut<PlayerHealth>,
) {
    burn_contacts.0.retain(|e| fire_query.contains(*e));

    if burn_contacts.0.is_empty() {
        timer.0.reset();
        return;
    }

    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        health.0 = health.0.saturating_sub(1);
    }
}

fn knocked_off_fire(p: f64, num_fires: u32, transform: &Transform, commands: &mut Commands) {
    let translation = transform.translation.truncate();

    for _ in 0..num_fires {
        let mut rng = rand::rng();
        if rng.random_bool(p) {
            let offset =
                Vec2::from_angle(rng.random_range(0.0..std::f32::consts::TAU)) * FIRE_SPAWN_DIST;
            let pos = translation + offset;
            info!("fire spawned at ({:.1}, {:.1})", pos.x, pos.y);
            commands.spawn(spawn_fire(pos));
        } else {
            info!("fire spawn rolled miss (p={p})");
        }
    }
}
