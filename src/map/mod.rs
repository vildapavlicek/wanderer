use crate::components::{Blocking, ItemName};
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

type TileSet = HashSet<Tile>;

struct Map {
    rooms: Vec<Room>,
    tiles: TileSet,
}

impl Map {
    fn new() -> Self {
        Map {
            rooms: vec![],
            tiles: TileSet::default(),
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

    let mut monster_spawner = monster_spawner::Spawner::new();
    monster_spawner.generate_monsters(&map.tiles);

    plug(&mut map);

    // spawn map
    for Tile { pos, kind } in map.tiles {
        match kind {
            TileType::Wall => {
                let r = rng.gen_range(0. ..=0.99);
                cmd.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: materials.cave_wall_sprite_sheet.clone(),
                    transform: Transform::from_xyz(
                        to_coords(pos.x),
                        to_coords(pos.y),
                        MONSTER_LAYER,
                    ),
                    ..Default::default()
                })
                .insert(Blocking::wall())
                .insert(Timer::from_seconds(r, true));

                // cmd.spawn_bundle(SpriteBundle {
                //     material: materials.cave_wall.clone(),
                //     sprite: Sprite::new(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                //     transform: Transform::from_xyz(
                //         to_coords(pos.x),
                //         to_coords(pos.y),
                //         MONSTER_LAYER,
                //     ),
                //     ..Default::default()
                // })
                // .insert(Blocking::wall());
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

    monster_spawner.spawn_monsters(&mut cmd, materials);
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
    let mut to_insert = TileSet::default();
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
    fn create_rect_room(&self, tiles: &mut TileSet) {
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

    let t3 = Tile {
        pos: IVec2::new(4, 3),
        kind: TileType::Floor,
    };

    let t4 = Tile {
        pos: IVec2::new(4, 3),
        kind: TileType::Wall,
    };

    let mut hasher_1 = std::collections::hash_map::DefaultHasher::new();
    let mut hasher_2 = std::collections::hash_map::DefaultHasher::new();
    let mut hasher_3 = std::collections::hash_map::DefaultHasher::new();
    let mut hasher_4 = std::collections::hash_map::DefaultHasher::new();

    let t1_hash = {
        t1.hash(&mut hasher_1);
        hasher_1.finish()
    };

    let t2_hash = {
        t2.hash(&mut hasher_2);
        hasher_2.finish()
    };

    let t3_hash = {
        t3.hash(&mut hasher_3);
        hasher_3.finish()
    };

    let t4_hash = {
        t4.hash(&mut hasher_4);
        hasher_4.finish()
    };

    assert_eq!(t1_hash, t2_hash, "first test failed");
    assert_ne!(t1_hash, t3_hash, "secont test failed");
    assert_eq!(t3_hash, t4_hash, "third test failed")
}

mod monster_spawner {
    use super::*;
    use crate::components::npc::MonsterStrength;
    use big_brain::pickers::FirstToScore;
    use big_brain::prelude::Thinker;

    type MonsterSet = HashSet<Monster>;

    const MONSTER_WEIGHTS: [(MonsterStrength, u32); 7] = [
        (MonsterStrength::Weak, 1),
        (MonsterStrength::Normal, 1),
        (MonsterStrength::Strong, 1),
        (MonsterStrength::Elite, 1),
        (MonsterStrength::Veteran, 1),
        (MonsterStrength::Leader, 1),
        (MonsterStrength::Boss, 1),
    ];

    fn monster_weight(monster_strength: MonsterStrength) -> i32 {
        match monster_strength {
            MonsterStrength::Weak => 2,
            MonsterStrength::Normal => 3,
            MonsterStrength::Strong => 4,
            MonsterStrength::Elite => 5,
            MonsterStrength::Veteran => 6,
            MonsterStrength::Leader => 7,
            MonsterStrength::Boss => 10,
        }
    }

    pub(super) struct Spawner {
        monster_count: i32,
        weak: i32,
        normal: i32,
        strong: i32,
        elite: i32,
        veteran: i32,
        boss: i32,
        monster_set: MonsterSet,
    }

    impl Spawner {
        pub(super) fn new() -> Self {
            Spawner {
                monster_count: 0,
                weak: 0,
                normal: 0,
                strong: 0,
                elite: 0,
                veteran: 0,
                boss: 0,
                monster_set: Default::default(),
            }
        }

        pub(super) fn generate_monsters(&mut self, tiles: &TileSet) {
            trace!("generating monsters");
            let tiles = tiles.iter().collect::<Vec<&'_ Tile>>();
            let max_index = tiles.len();
            // total_monster_weight should be used to count how many monsters we want to spawn
            let mut total_monster_weight = ((max_index as f32) * 0.5) as i32; // let's make sure that at least 25% of the map is walkable

            debug!(%max_index, %total_monster_weight, "constraints");

            let mut rng = rand::thread_rng();

            while total_monster_weight >= 0 {
                let index = rng.gen_range(0..max_index);

                if let Some(tile) = tiles.get(index) {
                    trace!(%index, ?tile, "got tile");
                    let monster = Monster::new(tile.pos, self.decide_monster_strength());
                    let weight = monster.weight() * 2;
                    if self.monster_set.insert(monster) {
                        total_monster_weight -= weight;
                        self.monster_count += 1;
                        debug!(%total_monster_weight, ?monster, "inserted monster");
                    }
                }
            }
        }

        pub(super) fn spawn_monsters(&self, cmd: &mut Commands, materials: Res<Materials>) {
            let mut rng = rand::thread_rng();

            for monster in &self.monster_set {
                let r = rng.gen_range(0. ..1.);

                match r {
                    _ if (0. ..0.25).contains(&r) => cmd
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: materials.flamey_sprite_sheet.clone(),
                            transform: Transform::from_xyz(
                                to_coords(monster.pos.x),
                                to_coords(monster.pos.y),
                                MONSTER_LAYER,
                            ),
                            ..Default::default()
                        })
                        .insert(Timer::from_seconds(0.1, true))
                        .insert_bundle(crate::components::npc::MeleeEnemy::new(
                            "Flamey".into(),
                            5,
                            crate::components::Race::Elemental,
                            1,
                            crate::components::Stats::new(1, 1, 1, 1),
                        ))
                        .insert(
                            Thinker::build()
                                .picker(FirstToScore { threshold: 0.95 })
                                .when(
                                    crate::ai::scorers::PlayerDistance::build(),
                                    crate::ai::actions::Move::build(),
                                )
                                .otherwise(crate::ai::actions::Skip::build()),
                        ),
                    _ if (0.25..0.6).contains(&r) => cmd
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                            material: materials.mole.clone(),
                            transform: Transform::from_xyz(
                                to_coords(monster.pos.x),
                                to_coords(monster.pos.y),
                                MONSTER_LAYER,
                            ),
                            ..Default::default()
                        })
                        .insert_bundle(crate::components::npc::MeleeEnemy::new(
                            "Cave MOLE".into(),
                            5,
                            crate::components::Race::Unknown,
                            1,
                            crate::components::Stats::new(1, 1, 1, 1),
                        ))
                        .insert(
                            Thinker::build()
                                .picker(FirstToScore { threshold: 0.95 })
                                .when(
                                    crate::ai::scorers::PlayerDistance::build(),
                                    crate::ai::actions::Move::build(),
                                )
                                .otherwise(crate::ai::actions::Skip::build()),
                        ),
                    _ => cmd
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                            material: materials.cave_spider.clone(),
                            transform: Transform::from_xyz(
                                to_coords(monster.pos.x),
                                to_coords(monster.pos.y),
                                MONSTER_LAYER,
                            ),
                            ..Default::default()
                        })
                        .insert_bundle(crate::components::npc::MeleeEnemy::new(
                            "Cave Spider".into(),
                            5,
                            crate::components::Race::Unknown,
                            1,
                            crate::components::Stats::new(1, 1, 1, 1),
                        ))
                        .insert(
                            Thinker::build()
                                .picker(FirstToScore { threshold: 0.95 })
                                .when(
                                    crate::ai::scorers::PlayerDistance::build(),
                                    crate::ai::actions::Move::build(),
                                )
                                .otherwise(crate::ai::actions::Skip::build()),
                        ),
                };
            }
        }

        fn decide_monster_strength(&self) -> MonsterStrength {
            if self.monster_count == 0 {
                return MonsterStrength::Weak;
            }
            let count = self.monster_count;
            match count {
                0 => MonsterStrength::Weak,
                _ if count % 33 == 0 => MonsterStrength::Veteran,
                _ if count % 13 == 0 => MonsterStrength::Elite,
                _ if count % 7 == 0 => MonsterStrength::Strong,
                _ if count % 3 == 0 => MonsterStrength::Normal,
                _ => MonsterStrength::Weak,
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Monster {
        strength: MonsterStrength,
        pos: IVec2,
    }

    impl Monster {
        fn new(pos: IVec2, strength: MonsterStrength) -> Self {
            Monster { strength, pos }
        }

        fn weight(&self) -> i32 {
            monster_weight(self.strength)
        }
    }

    impl Eq for Monster {}
    impl PartialEq<Self> for Monster {
        fn eq(&self, other: &Self) -> bool {
            self.pos == other.pos
        }
    }

    impl Hash for Monster {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.pos.y.hash(state);
            self.pos.x.hash(state);
        }
    }

    #[test]
    fn test_hash_eq() {
        let m1 = Monster {
            pos: IVec2::new(3, 3),
            strength: MonsterStrength::Weak,
        };

        let m2 = Monster {
            pos: IVec2::new(3, 3),
            strength: MonsterStrength::Weak,
        };

        let m3 = Monster {
            pos: IVec2::new(3, 4),
            strength: MonsterStrength::Weak,
        };

        let m4 = Monster {
            pos: IVec2::new(3, 4),
            strength: MonsterStrength::Boss,
        };

        let mut hasher_1 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher_2 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher_3 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher_4 = std::collections::hash_map::DefaultHasher::new();

        let m1_hash = {
            m1.hash(&mut hasher_1);
            hasher_1.finish()
        };

        let m2_hash = {
            m2.hash(&mut hasher_2);
            hasher_2.finish()
        };

        let m3_hash = {
            m3.hash(&mut hasher_3);
            hasher_3.finish()
        };

        let m4_hash = {
            m4.hash(&mut hasher_4);
            hasher_4.finish()
        };

        assert_eq!(m1_hash, m2_hash, "first test failed");
        assert_ne!(m1_hash, m3_hash, "second test failed");
        assert_eq!(m3_hash, m4_hash, "third test failed")
    }
}
