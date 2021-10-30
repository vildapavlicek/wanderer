use crate::components::Blocking;
use crate::resources::Materials;
use bevy::prelude::*;
use bevy::utils::HashSet;
use num_integer::Integer;
use rand::Rng;
use std::borrow::BorrowMut;

const SPRITE_SIZE: f32 = 32.;
const FLOOR_LAYER: f32 = 0.;
const ITEM_LAYER: f32 = 1.;
const MONSTER_LAYER: f32 = 2.;
const PLAYER_LAYER: f32 = 3.;

const MOVE_SIZE: f32 = 32.;

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

/// System that generates simple rooms and connects them. Always generates room with center at 0,0, so player always starts inside a room.
pub fn generate_map(mut cmd: Commands, materials: Res<Materials>) {
    let mut map = Map::new();
    let mut room = Room::new(IVec2::new(0, 0), 5, 5);
    room.create_rect_room(&mut map.tiles);
    map.rooms.push(room);

    // ---------------------- RNG rooms
    let mut rng = rand::thread_rng();
    let n_rooms = rng.gen_range(5..=10);

    for _ in 0..=n_rooms {
        let x = rng.gen_range(-25..=25);
        let y = rng.gen_range(-25..=25);
        let width = rng.gen_range(3..=10);
        let height = rng.gen_range(3..=10);

        map.rooms.push(Room::new(IVec2::new(x, y), height, width));
    }

    for room in map.rooms.iter_mut() {
        room.create_rect_room(&mut map.tiles);
    }

    connect_rooms(&mut map);
    plug(&mut map);

    // spawn map
    for Tile { pos, kind } in map.tiles {
        match kind {
            TileType::Wall => {
                cmd.spawn_bundle(SpriteBundle {
                    material: materials.obstacle_material.clone(),
                    sprite: Sprite::new(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                    transform: Transform::from_xyz(
                        to_coords(pos.x),
                        to_coords(pos.y),
                        MONSTER_LAYER,
                    ),
                    ..Default::default()
                })
                .insert(Blocking::obstacle());
            }
            TileType::Floor => {
                cmd.spawn_bundle(SpriteBundle {
                    material: materials.floor_material.clone(),
                    sprite: Sprite::new(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                    transform: Transform::from_xyz(to_coords(pos.x), to_coords(pos.y), FLOOR_LAYER),
                    ..Default::default()
                });
            }
        }
    }
}

/// Iterates over all rooms and connect one to next. This way we can be sure all rooms are connected
fn connect_rooms(map: &mut Map) {
    let rooms = map.rooms.as_slice();
    let tiles = map.tiles.borrow_mut();

    let mut iter = rooms.iter().peekable();
    while let Some(room) = iter.next() {
        if let Some(entry_points) = iter.peek().map(|next_room| next_room.center) {
            let start = room.center;
            let finish = entry_points;

            let (mut offset_x, mut offset_y) = (start.x, start.y);

            while offset_x != finish.x {
                let floor = Tile::floor(offset_x, start.y);
                let top_wall = Tile::wall(offset_x, start.y + 1);
                let bottom_wall = Tile::wall(offset_x, start.y - 1);

                tiles.replace(floor);

                if finish.x > offset_x {
                    offset_x += 1;
                } else {
                    offset_x -= 1;
                }
            }

            while offset_y != finish.y {
                let floor = Tile::floor(offset_x, offset_y);
                let left_wall = Tile::wall(offset_x - 1, offset_y);
                let right_wall = Tile::wall(offset_x + 1, offset_y);

                tiles.replace(floor);

                if finish.y > offset_y {
                    offset_y += 1;
                } else {
                    offset_y -= 1;
                }
            }
        }
    }
}

/// Iterates over our floor tiles and if is has no neigbohouring floor tile inserts wall tile
fn plug(map: &mut Map) {
    let mut to_insert = HashSet::default();
    for tile in map.tiles.iter() {
        // due to borrow checker we cannot iterate and also insert
        // also this is called when only floor tiles are generated, so we do not need to filter anything
        let pos = tile.pos;
        let left = Tile::wall(pos.x - 1, pos.y);
        let top = Tile::wall(pos.x, pos.y + 1);
        let top_left = Tile::wall(pos.x - 1, pos.y + 1);
        let right = Tile::wall(pos.x + 1, pos.y);
        let top_right = Tile::wall(pos.x + 1, pos.y + 1);
        let bottom = Tile::wall(pos.x, pos.y - 1);
        let bottom_left = Tile::wall(pos.x - 1, pos.y - 1);
        let bottom_right = Tile::wall(pos.x + 1, pos.y - 1);

        to_insert.insert(left);
        to_insert.insert(top);
        to_insert.insert(right);
        to_insert.insert(bottom);
        to_insert.insert(bottom_left);
        to_insert.insert(bottom_right);
        to_insert.insert(top_left);
        to_insert.insert(top_right);
    }

    to_insert.into_iter().for_each(|tile| {
        map.tiles.insert(tile);
    });
}

/// This represents a room. With center, height and width it should be easy to deduce all other things if needed
#[derive(Debug)]
struct Room {
    height: u32,
    width: u32,
    center: IVec2,
}

impl Room {
    /// Creates a new room
    fn new(center: IVec2, height: u32, width: u32) -> Self {
        Room {
            height,
            width,
            center,
        }
    }

    /// Creates a room with rectangular shape
    fn create_rect_room(&self, tiles: &mut HashSet<Tile>) {
        let start_x = self.center.x - (self.width as i32 / 2) - 1;
        let start_y = self.center.y - (self.height as i32 / 2) - 1;
        let width = self.width as i32;
        let height = self.height as i32;

        for i in 1..=height {
            // now spawn our floors
            for j in 1..=width {
                tiles.replace(Tile::floor(start_x + j, start_y + i));
            }
        }
    }
}

/// Converts position to game coordinates.
fn to_coords(x: i32) -> f32 {
    x as f32 * 32.
}

/// Tile type so we can differentiate and then spawn with correct assest
#[derive(Debug)]
enum TileType {
    Wall,
    Floor,
}

/// Represents single tile or single grind
#[derive(Debug)]
struct Tile {
    pos: IVec2,
    kind: TileType,
}

impl Tile {
    /// Creates new tile
    fn new(x: i32, y: i32, kind: TileType) -> Self {
        Tile {
            pos: IVec2::new(x, y),
            kind,
        }
    }
    /// Creates tile which represents wall
    fn wall(x: i32, y: i32) -> Self {
        Self::new(x, y, TileType::Wall)
    }

    /// Creates tile which represents floor
    fn floor(x: i32, y: i32) -> Self {
        Self::new(x, y, TileType::Floor)
    }
}

use std::hash::{Hash, Hasher};
impl std::hash::Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.pos.x as i32).hash(state);
        (self.pos.y as i32).hash(state);
    }
}

impl PartialEq<Self> for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl std::cmp::Eq for Tile {}

#[test]
fn test_hash_eq() {
    let t1 = Tile {
        pos: IVec2::new(3, 3),
        kind: TileType::Floor,
    };

    let t2 = Tile {
        pos: IVec2::new(3, 3),
        kind: TileType::Wall,
    };

    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    assert_eq!(t1.hash(&mut hasher), t2.hash(&mut hasher))
}
