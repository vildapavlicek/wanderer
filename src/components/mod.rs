pub mod npc;
pub mod player;

use bevy::prelude::{Bundle, Component};
use std::fmt::Formatter;
use std::ops::Add;

#[derive(Debug, Component)]
pub struct Timer(pub bevy::prelude::Timer);

/// This represents name of the thing, NPC or anything that needs to be named
#[derive(Debug, Component)]
pub struct ItemName(pub String);
impl std::fmt::Display for ItemName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Component)]
pub struct Level(pub i32);

#[derive(Debug, Component)]
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

/// This is component that marks that it will block your path
/// To make it easier it has type of blocking. If you are blocked by enemy (you'd bump into them) it should be instead considered as attack
#[derive(Debug, Component)]
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
        matches!(self.blocking_type, BlockingType::Enemy)
    }
}

#[derive(Debug, Component)]
pub enum BlockingType {
    Wall,
    Obstacle,
    Enemy,
    Player,
}

/// Component that stores max possible health as well as tracks the current health
#[derive(Debug, Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
    pub min: i32,
}

impl Health {
    pub fn to_ui_format(&self) -> String {
        format!("{} / {}", self.current, self.max)
    }
}

impl Health {
    pub fn new(max: i32, min: i32) -> Self {
        Health {
            current: max,
            max,
            min,
        }
    }
}

/// Compotent that should be attached to something that is regarded to be enemy to player. Should go in hand with Blocking::enemy()
#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug, Component)]
pub enum Race {
    Unknown,
    Human,
    Elf,
    Orc,
    Goblin,
    Elemental,
}

impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Human => write!(f, "Human"),
            Self::Elf => write!(f, "Elf"),
            Self::Orc => write!(f, "Orc"),
            Self::Goblin => write!(f, "Goblin"),
            Self::Elemental => write!(f, "Elemental"),
        }
    }
}

/// Rarity types of items that can be found. Each rarity level aslo shoud add some bonuses
#[derive(Debug, Component)]
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

#[derive(Debug, Component)]
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

macro_rules! stat_inner {
    ($stat: ident) => {
        impl $stat {
            pub fn inner(&self) -> usize {
                self.0
            }
        }
    };
}

#[derive(Debug, Component)]
pub struct Strength(usize);
stat_inner!(Strength);
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

#[derive(Debug, Component)]
pub struct Agility(usize);
stat_inner!(Agility);
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

#[derive(Debug, Component)]
pub struct Endurance(usize);
stat_inner!(Endurance);
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

#[derive(Debug, Component)]
pub struct Intelligence(usize);
stat_inner!(Intelligence);
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

#[derive(Debug, Bundle)]
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

#[derive(Debug, Component)]
pub enum WeaponBonusType {
    Strength(Strength),
    Agility(Agility),
    Endurance(Endurance),
    Intelligence(Intelligence),
}

#[derive(Debug, Component)]
pub enum WeaponRangeType {
    Ranged(i32, i32),
    Melee,
    Aoe,
}

#[derive(Debug, Component)]
pub struct Weapon {
    name: String,
    attack_range: WeaponRangeType,
    rarity: Rarity,
    min: usize,
    max: usize,
    bonuses: Option<Vec<WeaponBonusType>>,
}

#[derive(Debug, Component)]
pub enum ArmorType {
    Head,
    Torso,
    Hands,
    Legs,
    Feet,
}

#[derive(Debug, Component)]
pub struct Armor {
    name: String,
    rarity: Rarity,
    kind: ArmorType,
    defense: usize,
}

#[derive(Debug, Component)]
pub struct Dead;
