use crate::components::Blocking;
use bevy::prelude::Bundle;

pub struct Player;
pub struct PlayerCamera;

pub const PLAYER_MAX_HEALTH: usize = 100;

pub struct XP {
    current: usize,
    max: usize,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    _p: Player,
    race: super::Race,
    level: super::Level,
    xp: XP,
    health: super::Health,
    blocking: super::Blocking,
    state: super::State,
}

impl PlayerBundle {
    pub fn new(max_health: i32) -> Self {
        PlayerBundle {
            _p: Player,
            race: super::Race::Unknown,
            level: super::Level(1),
            xp: XP {
                current: 0,
                max: PLAYER_MAX_HEALTH,
            },
            health: super::Health::new(max_health),
            blocking: super::Blocking::player(),
            state: super::State::default(),
        }
    }
}
