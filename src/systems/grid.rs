use crate::components::{Blocking, Enemy, Health};
use crate::resources::Materials;
use crate::systems::enemy::MoveDirection;
use bevy::prelude::*;
use rand::Rng;

const FLOOR: char = '.';
const WALL: char = '|';
const ENEMY: char = 'e';

pub struct Map {
    pub x_size: usize,
    pub y_size: usize,
    layout: Vec<char>,
}

impl Map {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        let mut vec = Vec::with_capacity(x_size * y_size);

        for _ in 0..(x_size * y_size) {
            let n = rand::thread_rng().gen_range(0.0..=1.0);
            match n {
                _ if n > 0.9 => vec.push(ENEMY),
                _ if n < 0.1 => vec.push(WALL),
                _ => vec.push(FLOOR),
            }
        }

        Map {
            x_size,
            y_size,
            layout: vec,
        }
    }
}
pub fn generate_map(mut cmd: Commands, materials: Res<Materials>) {
    let map = Map::new(20, 20);

    for (idx, char) in map.layout.iter().enumerate() {
        let (x, y) = idx_to_pos(idx, map.x_size);

        // we want to always spawn floor!
        cmd.spawn_bundle(SpriteBundle {
            material: materials.floor_material.clone(),
            sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
            transform: Transform::from_xyz(to_coords(x), to_coords(y), super::FLOOR_LAYER),
            ..Default::default()
        });

        match *char {
            ENEMY => {
                cmd.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: materials.flamey_sprite_sheet.clone(),
                    // sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                    transform: Transform::from_xyz(
                        to_coords(x),
                        to_coords(y),
                        super::MONSTER_LAYER,
                    ),
                    ..Default::default()
                })
                .insert(MoveDirection::Right) // todo remove for testing only
                .insert(Timer::from_seconds(0.1, true))
                .insert_bundle(crate::components::npc::MeeleeEnemy::new(
                    "Flamey".into(),
                    5,
                    crate::components::Race::Elemental,
                    1,
                    crate::components::Stats::new(1, 1, 1, 1),
                ));
            }
            WALL => {
                cmd.spawn_bundle(SpriteBundle {
                    material: materials.obstacle_material.clone(),
                    sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                    transform: Transform::from_xyz(
                        to_coords(x),
                        to_coords(y),
                        super::MONSTER_LAYER,
                    ),
                    ..Default::default()
                })
                .insert(Blocking::obstacle());
            }
            _ => (),
        }
    }

    cmd.insert_resource(map);
}

fn idx_to_pos(idx: usize, x_size: usize) -> (i32, i32) {
    ((idx % x_size) as i32, (idx / x_size) as i32)
}

fn to_coords(x: i32) -> f32 {
    x as f32 * 32.
}
