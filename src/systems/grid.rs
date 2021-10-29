use crate::components::Blocking;
use crate::resources::Materials;
// use crate::systems::enemy::MoveDirection;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::Rng;

struct Map {
    rooms: Vec<Room>,
    tiles: HashSet<Tile>,
}

impl Map {
    fn new() -> Self {
        Map {
            rooms: vec![],
            tiles: HashSet::default(),
        }
    }
}

pub fn generate_map(mut cmd: Commands, materials: Res<Materials>) {
    let mut map = Map::new();
    let mut room = Room::new(IVec2::new(0, 0), 3, 11);
    room.create_rect_room(&mut map);
    room.create_entry_top(&mut map);
    room.create_entry_left(&mut map);
    room.create_entry_bottom(&mut map);
    room.create_entry_right(&mut map);

    let mut room2 = Room::new(IVec2::new(0, 16), 5, 5);
    room2.create_rect_room(&mut map);
    room2.create_entry_top(&mut map);
    room2.create_entry_left(&mut map);
    room2.create_entry_bottom(&mut map);
    room2.create_entry_right(&mut map);

    for Tile { coords, kind } in map.tiles {
        match kind {
            TileType::Wall => {
                cmd.spawn_bundle(SpriteBundle {
                    material: materials.obstacle_material.clone(),
                    sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                    transform: Transform::from_xyz(coords.x, coords.y, super::MONSTER_LAYER),
                    ..Default::default()
                })
                .insert(Blocking::obstacle());
            }
            TileType::Floor => {
                cmd.spawn_bundle(SpriteBundle {
                    material: materials.floor_material.clone(),
                    sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                    transform: Transform::from_xyz(coords.x, coords.y, super::FLOOR_LAYER),
                    ..Default::default()
                });
            }
        }
    }
}

struct Room {
    entry_points: Vec<IVec2>,
    height: u32,
    width: u32,
    center: IVec2,
}

impl Room {
    fn new(center: IVec2, height: u32, width: u32) -> Self {
        Room {
            entry_points: vec![],
            height,
            width,
            center,
        }
    }

    fn create_rect_room(&self, map: &mut Map) {
        println!("trying to spawn room");
        let start_x = (self.center.x - (self.width as i32 / 2) - 1);
        let start_y = (self.center.y - (self.height as i32 / 2) - 1);
        let width = self.width as i32;
        let height = self.height as i32;

        println!("spawning bottom walls");
        for i in 0..=width + 1 {
            map.tiles.insert(Tile {
                coords: Vec2::new(to_coords(start_x + i) as f32, to_coords(start_y) as f32),
                kind: TileType::Wall,
            });
        }

        println!("spawning rest of room");
        for i in 1..=height {
            map.tiles.insert(Tile::wall(start_x, start_y + i));
            // now spawn our floors
            for j in 1..=width {
                map.tiles.insert(Tile::floor(start_x + j, start_y + i));
            }

            // spawn our right wall
            map.tiles
                .insert(Tile::wall(start_x + width + 1, start_y + i));
        }

        for i in 0..=width + 1 {
            map.tiles
                .insert(Tile::wall(start_x + i, start_y + height + 1));
        }

        println!("room generation finished")
    }

    fn create_entry_top(&mut self, map: &mut Map) {
        let pos = IVec2::new(
            self.center.x,
            (self.center.y + (self.height as i32 / 2) + 1),
        );
        let tile = Tile::floor(pos.x, pos.y);
        let x = map.tiles.replace(tile);
        self.entry_points.push(pos);
        println!("replaced: {:?}, pos: {:?}", x, pos);
    }

    fn create_entry_bottom(&mut self, map: &mut Map) {
        let pos = IVec2::new(
            self.center.x,
            (self.center.y - (self.height as i32 / 2) - 1),
        );
        let tile = Tile::floor(pos.x, pos.y);
        let x = map.tiles.replace(tile);
        self.entry_points.push(pos);
        println!("replaced: {:?}, pos: {:?}", x, pos);
    }

    fn create_entry_left(&mut self, map: &mut Map) {
        let pos = IVec2::new(self.center.x - (self.width as i32 / 2) - 1, self.center.y);
        let tile = Tile::floor(pos.x, pos.y);
        let x = map.tiles.replace(tile);
        self.entry_points.push(pos);
        println!("replaced: {:?}, pos: {:?}", x, pos);
    }

    fn create_entry_right(&mut self, map: &mut Map) {
        let pos = IVec2::new(self.center.x + (self.width as i32 / 2) + 1, self.center.y);
        let tile = Tile::floor(pos.x, pos.y);
        let x = map.tiles.replace(tile);
        self.entry_points.push(pos);
        println!("replaced: {:?}, pos: {:?}", x, pos);
    }
}

fn to_coords(x: i32) -> f32 {
    x as f32 * 32.
}

#[derive(Debug)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
struct Tile {
    coords: Vec2,
    kind: TileType,
}

impl Tile {
    fn wall(x: i32, y: i32) -> Self {
        Tile {
            coords: Vec2::new(to_coords(x), to_coords(y)),
            kind: TileType::Wall,
        }
    }

    fn floor(x: i32, y: i32) -> Self {
        Tile {
            coords: Vec2::new(to_coords(x), to_coords(y)),
            kind: TileType::Floor,
        }
    }
}

use std::hash::{Hash, Hasher};
impl std::hash::Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.coords.x as i32).hash(state);
        (self.coords.y as i32).hash(state);
    }
}

impl PartialEq<Self> for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl std::cmp::Eq for Tile {}

#[test]
fn test_hash_eq() {
    let t1 = Tile {
        coords: Vec2::new(3., 3.),
        kind: TileType::Floor,
    };

    let t2 = Tile {
        coords: Vec2::new(3., 3.),
        kind: TileType::Wall,
    };

    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    assert_eq!(t1.hash(&mut hasher), t2.hash(&mut hasher))
}
