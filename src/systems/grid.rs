use crate::components::{Blocking, Enemy, Health};
use crate::resources::Materials;
use crate::systems::enemy::MoveDirection;
use bevy::prelude::*;
use rand::Rng;

// pub const ARENA_WIDTH: u32 = 10;
// pub const ARENA_HEIGHT: u32 = 10;
//
// const ORIGIN_OFFSET_X: usize = 320;
// const ORIGIN_OFFSET_Y: usize = 320;

// this will inevitable size the game area to whole window which I do not want
// pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
//     let window = windows.get_primary().unwrap();
//     for (sprite_size, mut sprite) in q.iter_mut() {
//         sprite.size = Vec2::new(
//             sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
//             sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
//         );
//     }
// }

// pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
//     fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
//         let tile_size = bound_window / bound_game;
//         pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
//     }
//     let window = windows.get_primary().unwrap();
//     for (pos, mut transform) in q.iter_mut() {
//         let x = convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32);
//         let y = convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32);
//         transform.translation = Vec3::new(x, y, transform.translation.z);
//     }
// }

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
                _ if n < 0.4 => vec.push(WALL),
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
        let (x, y) = idx_to_pos(idx, map.x_size, map.y_size);

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
                .insert(Blocking::enemy())
                .insert(Health::new(2))
                .insert(Enemy)
                .insert(MoveDirection::Right)
                .insert(Timer::from_seconds(0.1, true));
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

fn idx_to_pos(idx: usize, x_size: usize, y_size: usize) -> (i32, i32) {
    ((idx % x_size) as i32, (idx / y_size) as i32)
}

fn to_coords(x: i32) -> f32 {
    x as f32 * 32. /*- 16.*/
}
