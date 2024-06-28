use crate::components::Blocking;
use bevy::prelude::{Bundle, Component};

#[derive(Debug, Component)]
pub struct Player;
#[derive(Debug, Component)]
pub struct PlayerCamera;

pub const PLAYER_MAX_HEALTH: usize = 100;

#[derive(Debug, Component)]
pub struct XP {
    current: usize,
    max: usize,
}

#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    _p: Player,
    name: super::ItemName,
    race: super::Race,
    level: super::Level,
    xp: XP,
    health: super::Health,
    blocking: super::Blocking,
    state: super::State,

    stats: super::Stats,
}

impl PlayerBundle {
    pub fn new(max_health: i32) -> Self {
        PlayerBundle {
            _p: Player,
            name: super::ItemName("ReadyPlayer1".into()),
            race: super::Race::Unknown,
            level: super::Level(1),
            xp: XP {
                current: 0,
                max: PLAYER_MAX_HEALTH,
            },
            health: super::Health::new(max_health, 0),
            blocking: super::Blocking::player(),
            state: super::State::default(),
            stats: super::Stats::new(10, 5, 8, 3),
        }
    }
}
