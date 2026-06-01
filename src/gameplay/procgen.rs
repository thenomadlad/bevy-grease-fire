use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::seq::IndexedRandom;

/// TODO: ObjectKind should be dynamically loaded by reading the available sprite-sheets in a
/// particular directory in assets/ or something like that

#[derive(Debug, Clone)]
pub enum ObjectKind {
    Table,
    Chair,
    KitchenCounter,
    Fridge,
    Stove,
}

const ALL_KINDS: &[ObjectKind] = &[
    ObjectKind::Table,
    ObjectKind::Chair,
    ObjectKind::KitchenCounter,
    ObjectKind::Fridge,
    ObjectKind::Stove,
];

#[derive(Debug)]
pub struct PlacedObject {
    pub kind: ObjectKind,
    pub position: Vec2,
}

pub trait RoomGenerator {
    fn generate(&self, bounds: Rect, seed: u32, cell_size: f32) -> Vec<PlacedObject>;
}

pub struct NoiseScatterGenerator {
    pub density: f64,
    pub exclusion_rects: Vec<Rect>,
}

// Controls how broadly noise varies across the grid. Smaller = larger furniture clusters.
const NOISE_SCALE: f64 = 0.3;

impl RoomGenerator for NoiseScatterGenerator {
    fn generate(&self, bounds: Rect, seed: u32, cell_size: f32) -> Vec<PlacedObject> {
        let mut rng = rand::rng();
        let noise = Perlin::new(seed);

        iter_cells(bounds, cell_size)
            .filter_map(|(row_idx, col_idx)| {
                let raw = noise.get([row_idx as f64 * NOISE_SCALE, col_idx as f64 * NOISE_SCALE]);
                // Normalize from [-1, 1] to [0, 1] so density=0.5 ≈ 50% fill.
                let p = (raw + 1.0) / 2.0;
                let selected_object = if p < self.density {
                    ALL_KINDS.choose(&mut rng).cloned()
                } else {
                    None
                };

                selected_object.and_then(|kind| {
                    let position = Vec2::new(
                        bounds.min.x + row_idx as f32 * cell_size + cell_size / 2.0,
                        bounds.min.y + col_idx as f32 * cell_size + cell_size / 2.0,
                    );
                    if self.exclusion_rects.iter().any(|r| r.contains(position)) {
                        return None;
                    }
                    Some(PlacedObject { kind, position })
                })
            })
            .collect()
    }
}

fn iter_cells(bounds: Rect, cell_size: f32) -> impl Iterator<Item = (usize, usize)> {
    let rows = (bounds.width() / cell_size).ceil() as usize;
    let cols = (bounds.height() / cell_size).ceil() as usize;

    (0..rows).flat_map(move |row_idx| (0..cols).map(move |col_idx| (row_idx, col_idx)))
}
