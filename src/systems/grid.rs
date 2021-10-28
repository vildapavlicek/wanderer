use crate::components::Blocking;
use crate::resources::Materials;
// use crate::systems::enemy::MoveDirection;
use bevy::prelude::*;
use rand::Rng;

/* const FLOOR: char = '.';
const WALL: char = '|';
const ENEMY: char = 'e';
 */
/* pub struct Map {
    pub x_size: usize,
    pub y_size: usize,
    layout: Vec<char>,
}

impl Map {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        /* let mut vec = Vec::with_capacity(x_size * y_size);

        for _ in 0..(x_size * y_size) {
            let n = rand::thread_rng().gen_range(0.0..=1.0);
            match n {
                _ if n > 0.9 => vec.push(ENEMY),
                _ if n < 0.1 => vec.push(WALL),
                _ => vec.push(FLOOR),
            }
        } */

        Map {
            x_size,
            y_size,
            layout: vec![],
        }
    }
} */

pub fn generate_map(mut cmd: Commands, materials: Res<Materials>) {
    // let map = Map::new(20, 20);
    spawn_rect_room(5, 5, 0, 0, &mut cmd, &materials);

    // cmd.insert_resource(map);
}

fn spawn_rect_room(
    height: i32,
    width: i32,
    center_x: i32,
    center_y: i32,
    cmd: &mut Commands,
    materials: &Res<Materials>,
) {
    println!("trying to spawn room");
    let start_x = center_x - (width / 2) - 1;
    let start_y = center_y - (height / 2) - 1;

    println!("spawning bottom walls");
    for i in 0..=width + 1 {
        // here we spawn bottom walls
        cmd.spawn_bundle(SpriteBundle {
            material: materials.obstacle_material.clone(),
            sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
            transform: Transform::from_xyz(
                to_coords(start_x + i),
                to_coords(start_y),
                super::MONSTER_LAYER,
            ),
            ..Default::default()
        })
        .insert(Blocking::obstacle());
    }

    println!("spawning rest of room");
    for i in 1..=height {
        // our first row is already spawned so we start at 1 to offset
        // spawn left wall
        cmd.spawn_bundle(SpriteBundle {
            material: materials.obstacle_material.clone(),
            sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
            transform: Transform::from_xyz(
                to_coords(start_x),
                to_coords(start_y + i),
                super::MONSTER_LAYER,
            ),
            ..Default::default()
        })
        .insert(Blocking::obstacle());

        // now spawn our floors
        for j in 1..=width {
            // first in the row is wall already spawned, so we start at 1 to offset
            cmd.spawn_bundle(SpriteBundle {
                material: materials.floor_material.clone(),
                sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                transform: Transform::from_xyz(
                    to_coords(start_x + j),
                    to_coords(start_y + i),
                    super::FLOOR_LAYER,
                ),
                ..Default::default()
            });
        }

        // spawn our right wall
        cmd.spawn_bundle(SpriteBundle {
            material: materials.obstacle_material.clone(),
            sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
            transform: Transform::from_xyz(
                to_coords(start_x + width + 1),
                to_coords(start_y + i),
                super::MONSTER_LAYER,
            ),
            ..Default::default()
        })
        .insert(Blocking::obstacle());
    }

    println!("spawning top walls");
    for i in 0..=width + 1 {
        // here we spawn top walls
        cmd.spawn_bundle(SpriteBundle {
            material: materials.obstacle_material.clone(),
            sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
            transform: Transform::from_xyz(
                to_coords(start_x + i),
                to_coords(start_y + height + 1),
                super::MONSTER_LAYER,
            ),
            ..Default::default()
        })
        .insert(Blocking::obstacle());
    }

    println!("room generation finished")
}

/* fn idx_to_pos(idx: usize, x_size: usize) -> (i32, i32) {
    ((idx % x_size) as i32, (idx / x_size) as i32)
}
 */
fn to_coords(x: i32) -> f32 {
    x as f32 * 32.
}
