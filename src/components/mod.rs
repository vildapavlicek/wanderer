pub mod npc;
pub mod player;

use bevy::prelude::Bundle;
use std::ops::Add;

#[derive(Debug)]
pub struct Name(String);
#[derive(Debug)]
pub struct Level(i32);

#[derive(Debug)]
pub enum State {
    Idle,
    Moving,
    Attacking,
}

impl std::default::Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug)]
pub struct Blocking {
    pub blocking_type: BlockingType,
}

impl Blocking {
    pub fn wall() -> Self {
        Self {
            blocking_type: BlockingType::Wall,
        }
    }

    pub fn obstacle() -> Self {
        Self {
            blocking_type: BlockingType::Obstacle,
        }
    }

    pub fn enemy() -> Self {
        Self {
            blocking_type: BlockingType::Enemy,
        }
    }

    pub fn player() -> Self {
        Self {
            blocking_type: BlockingType::Player,
        }
    }

    pub fn is_attackable(&self) -> bool {
        match self.blocking_type {
            BlockingType::Enemy => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum BlockingType {
    Wall,
    Obstacle,
    Enemy,
    Player,
}

#[derive(Debug)]
pub struct Health {
    pub current: i32,
    max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Health { current: max, max }
    }
}

pub struct Enemy;

#[derive(Debug)]
pub enum Race {
    Unknown,
    Human,
    Elf,
    Orc,
    Goblin,
    Elemental,
}

#[derive(Debug)]
pub enum Rarity {
    Damaged,
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn get_bonus(&self) -> i32 {
        match self {
            Rarity::Damaged => -2,
            Rarity::Common => 0,
            Rarity::Uncommon => 1,
            Rarity::Rare => 4,
            Rarity::Epic => 7,
            Rarity::Legendary => 10,
        }
    }
}

#[derive(Debug)]
pub enum WeaponType {
    Unarmed,
    Sword,
    TwoHandedSword,
    Mace,
    TwoHandedMace,
    Shield,
    Bow,
    Crossbow,
    Gun,
}

macro_rules! stat_fmt {
    ($stat: ident) => {
        impl std::fmt::Display for $stat {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", std::stringify!($stat), self.0)
            }
        }
    };
}

#[derive(Debug)]
pub struct Strength(usize);
impl Add<Strength> for usize {
    type Output = usize;

    fn add(self, rhs: Strength) -> Self::Output {
        self + rhs.0
    }
}

impl Add<Strength> for i32 {
    type Output = i32;

    fn add(self, rhs: Strength) -> Self::Output {
        self + rhs.0 as i32
    }
}

#[derive(Debug)]
pub struct Agility(usize);
impl Add<Agility> for usize {
    type Output = usize;

    fn add(self, rhs: Agility) -> Self::Output {
        self + rhs.0
    }
}

impl Add<Agility> for i32 {
    type Output = i32;

    fn add(self, rhs: Agility) -> Self::Output {
        self + rhs.0 as i32
    }
}

#[derive(Debug)]
pub struct Endurance(usize);
impl Add<Endurance> for usize {
    type Output = usize;

    fn add(self, rhs: Endurance) -> Self::Output {
        self + rhs.0
    }
}

impl Add<Endurance> for i32 {
    type Output = i32;

    fn add(self, rhs: Endurance) -> Self::Output {
        self + rhs.0 as i32
    }
}

#[derive(Debug)]
pub struct Intelligence(usize);
impl Add<Intelligence> for usize {
    type Output = usize;

    fn add(self, rhs: Intelligence) -> Self::Output {
        self + rhs.0
    }
}

impl Add<Intelligence> for i32 {
    type Output = i32;

    fn add(self, rhs: Intelligence) -> Self::Output {
        self + rhs.0 as i32
    }
}

stat_fmt!(Strength);
stat_fmt!(Agility);
stat_fmt!(Endurance);
stat_fmt!(Intelligence);

#[derive(Bundle)]
pub struct Stats {
    strength: Strength,
    agility: Agility,
    endurance: Endurance,
    intelligence: Intelligence,
}

impl Stats {
    pub fn new(strength: usize, agility: usize, endurance: usize, intelligence: usize) -> Self {
        Stats {
            strength: Strength(strength),
            agility: Agility(agility),
            endurance: Endurance(endurance),
            intelligence: Intelligence(intelligence),
        }
    }
}

#[derive(Debug)]
pub enum WeaponBonusType {
    Strength(Strength),
    Agility(Agility),
    Endurance(Endurance),
    Intelligence(Intelligence),
}

#[derive(Debug)]
pub enum WeaponRangeType {
    Ranged(i32, i32),
    Melee,
    Aoe,
}

#[derive(Debug)]
pub struct Weapon {
    name: String,
    attack_range: WeaponRangeType,
    rarity: Rarity,
    min: usize,
    max: usize,
    bonuses: Option<Vec<WeaponBonusType>>,
}

pub enum ArmorType {
    Head,
    Torso,
    Hands,
    Legs,
    Feet,
}

pub struct Armor {
    name: String,
    rarity: Rarity,
    kind: ArmorType,
    armor: usize,
}
