use crate::components::Blocking;
use crate::resources::Materials;
use bevy::prelude::*;
use bevy::utils::HashSet;
use num_integer::Integer;
use rand::Rng;
use std::borrow::BorrowMut;

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
    let mut room = Room::new(IVec2::new(0, 0), 5, 5);
    room.create_rect_room(&mut map.tiles);
    room.create_entry_top(&mut map.tiles);
    // room.create_entry_left(&mut map.tiles);
    //     room.create_entry_bottom(&mut map.tiles);
    // room.create_entry_right(&mut map.tiles);
    map.rooms.push(room);

    // let mut room = Room::new(IVec2::new(0, -10), 5, 5);
    // room.create_rect_room(&mut map.tiles);
    // room.create_entry_top(&mut map.tiles);
    // room.create_entry_left(&mut map.tiles);
    // room.create_entry_bottom(&mut map.tiles);
    // room.create_entry_right(&mut map.tiles);
    // map.rooms.push(room);

    // let mut room = Room::new(IVec2::new(0, 10), 5, 5);
    // room.create_rect_room(&mut map.tiles);
    // room.create_entry_top(&mut map.tiles);
    // room.create_entry_left(&mut map.tiles);
    // room.create_entry_bottom(&mut map.tiles);
    // room.create_entry_right(&mut map.tiles);
    // map.rooms.push(room);

    // ---------------------- RNG rooms
    let mut rng = rand::thread_rng();
    let n_rooms = rng.gen_range(10..=30);

    for _ in 0..=5
    /* n_rooms */
    {
        let x = rng.gen_range(-25..=25);
        let y = rng.gen_range(-25..=25);
        let width = rng.gen_range(3..=10);
        let height = rng.gen_range(3..=10);

        map.rooms.push(Room::new(IVec2::new(x, y), height, width));
    }

    for room in map.rooms.iter_mut() {
        room.create_rect_room(&mut map.tiles);
        let exit_position = rng.gen();
        room.create_exit_at(&mut map.tiles, exit_position);
    }

    connect_rooms(&mut map);

    // spawn map
    for Tile { pos, kind } in map.tiles {
        match kind {
            TileType::Wall => {
                cmd.spawn_bundle(SpriteBundle {
                    material: materials.obstacle_material.clone(),
                    sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                    transform: Transform::from_xyz(
                        to_coords(pos.x),
                        to_coords(pos.y),
                        super::MONSTER_LAYER,
                    ),
                    ..Default::default()
                })
                .insert(Blocking::obstacle());
            }
            TileType::Floor => {
                cmd.spawn_bundle(SpriteBundle {
                    material: materials.floor_material.clone(),
                    sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                    transform: Transform::from_xyz(
                        to_coords(pos.x),
                        to_coords(pos.y),
                        super::FLOOR_LAYER,
                    ),
                    ..Default::default()
                });
            }
        }
    }
}

fn connect_rooms(map: &mut Map) {
    let rooms = map.rooms.as_slice();
    let tiles = map.tiles.borrow_mut();

    let mut iter = rooms.iter().peekable();
    while let Some(room) = iter.next() {
        if let Some(entry_points) = iter.peek().map(
            |next_room| next_room.entry_points.clone(), /*todo get rid of this clone*/
        ) {
            let start = room
                .entry_points
                .first()
                .expect("entry point not found for room");
            let finish = entry_points
                .first()
                .expect("entry point not found for NEXT room");

            println!(
                "got room {:?} and next room. start entry point: '{:?}' and finish entry_point '{:?}'",
                room, start, finish
            );

            let (mut offset_x, mut offset_y) = (start.x, start.y);
            println!(
                "generating path with offset_x {} and offset_y {}",
                offset_x, offset_y
            );
            while offset_x != finish.x {
                println!(
                    "start.x {}, offset_x {}, finish.x {}",
                    start.x, offset_x, finish.x
                );
                let floor = Tile::floor(offset_x, start.y);
                let top_wall = Tile::wall(offset_x, start.y + 1);
                let bottom_wall = Tile::wall(offset_x, start.y - 1);

                tiles.replace(floor);
                tiles.insert(top_wall);
                tiles.insert(bottom_wall);

                if finish.x > offset_x {
                    offset_x += 1;
                } else {
                    offset_x -= 1;
                }
            }

            while offset_y != finish.y {
                println!(
                    "start.y {}, offset_y {}, finish.y {}",
                    start.y, offset_y, finish.y
                );
                /* let floor = Tile::floor(offset_x, start.y + offset_y);
                let left_wall = Tile::wall(offset_x - 1, start.y + offset_y);
                let right_wall = Tile::wall(offset_x + 1, start.y + offset_y); */

                let floor = Tile::floor(offset_x, offset_y);
                let left_wall = Tile::wall(offset_x - 1, offset_y);
                let right_wall = Tile::wall(offset_x + 1, offset_y);

                tiles.replace(floor);
                tiles.insert(left_wall);
                tiles.insert(right_wall);

                if finish.y > offset_y {
                    offset_y += 1;
                } else {
                    offset_y -= 1;
                }
            }
        }
    }
}

#[derive(Debug)]
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

    fn create_rect_room(&self, tiles: &mut HashSet<Tile>) {
        let start_x = self.center.x - (self.width as i32 / 2) - 1;
        let start_y = self.center.y - (self.height as i32 / 2) - 1;
        let width = self.width as i32;
        let height = self.height as i32;

        for i in 0..=width + 1 {
            tiles.replace(Tile::wall(start_x + i, start_y));
        }

        for i in 1..=height {
            tiles.replace(Tile::wall(start_x, start_y + i));
            // now spawn our floors
            for j in 1..=width {
                tiles.replace(Tile::floor(start_x + j, start_y + i));
            }

            // spawn our right wall
            tiles.replace(Tile::wall(start_x + width + 1, start_y + i));
        }

        for i in 0..=width + 1 {
            tiles.replace(Tile::wall(start_x + i, start_y + height + 1));
        }
    }

    fn create_entry_top(&mut self, tiles: &mut HashSet<Tile>) {
        let offset = i32::from(self.height.is_odd());
        let pos = IVec2::new(
            self.center.x,
            self.center.y + (self.height as i32 / 2) + offset,
        );
        let tile = Tile::floor(pos.x, pos.y);
        let x = tiles.replace(tile);
        self.entry_points.push(pos);
    }

    fn create_entry_bottom(&mut self, tiles: &mut HashSet<Tile>) {
        let pos = IVec2::new(self.center.x, self.center.y - (self.height as i32 / 2) - 1);
        let tile = Tile::floor(pos.x, pos.y);
        let x = tiles.replace(tile);
        self.entry_points.push(pos);
    }

    fn create_entry_left(&mut self, tiles: &mut HashSet<Tile>) {
        let pos = IVec2::new(self.center.x - (self.width as i32 / 2) - 1, self.center.y);
        let tile = Tile::floor(pos.x, pos.y);
        let x = tiles.replace(tile);
        self.entry_points.push(pos);
    }

    fn create_entry_right(&mut self, tiles: &mut HashSet<Tile>) {
        let offset = i32::from(self.width.is_odd());
        let pos = IVec2::new(
            self.center.x + (self.width as i32 / 2) + offset,
            self.center.y,
        );
        let tile = Tile::floor(pos.x, pos.y);
        let x = tiles.replace(tile);
        self.entry_points.push(pos);
    }

    fn create_entries(&mut self, tiles: &mut HashSet<Tile>) {
        self.create_rect_room(tiles);
        self.create_entry_top(tiles);
        self.create_entry_left(tiles);
        self.create_entry_bottom(tiles);
        self.create_entry_right(tiles);
    }

    fn create_exit_at(&mut self, tiles: &mut HashSet<Tile>, exit_pos: ExitPosition) {
        match exit_pos {
            ExitPosition::Left => self.create_entry_left(tiles),
            ExitPosition::Top => self.create_entry_top(tiles),
            ExitPosition::Right => self.create_entry_right(tiles),
            ExitPosition::Bottom => self.create_entry_bottom(tiles),
        }
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
    pos: IVec2,
    kind: TileType,
}

impl Tile {
    fn new(x: i32, y: i32, kind: TileType) -> Self {
        Tile {
            pos: IVec2::new(x, y),
            kind,
        }
    }
    fn wall(x: i32, y: i32) -> Self {
        Self::new(x, y, TileType::Wall)
    }

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

enum ExitPosition {
    Left,
    Top,
    Right,
    Bottom,
}

impl rand::distributions::Distribution<ExitPosition> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExitPosition {
        match rng.gen_range(0..4) {
            0 => ExitPosition::Left,
            1 => ExitPosition::Top,
            2 => ExitPosition::Right,
            3 => ExitPosition::Bottom,
            _ => ExitPosition::Left, // this should not happen
        }
    }
}

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
