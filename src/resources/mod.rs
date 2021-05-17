use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TempGameState {
    PlayerTurn,
    EnemyTurn,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum GameStateType {
    MainMenu,
    PlayerTurn,
    EnemyTurn,
    Targeting,
    Inventory,
}

impl std::default::Default for GameStateType {
    fn default() -> Self {
        GameStateType::PlayerTurn
    }
}

pub struct GameState {
    current: GameStateType,
}

impl GameState {
    pub fn next(&mut self) {
        self.current = match self.current {
            GameStateType::PlayerTurn => GameStateType::EnemyTurn,
            GameStateType::EnemyTurn => GameStateType::PlayerTurn,
            _ => GameStateType::PlayerTurn,
        }
    }

    pub fn current(&self) -> GameStateType {
        self.current
    }

    pub fn is_player_turn(&self) -> bool {
        self.current == GameStateType::PlayerTurn
    }

    pub fn is_enemy_turn(&self) -> bool {
        self.current == GameStateType::EnemyTurn
    }
}

impl std::default::Default for GameState {
    fn default() -> Self {
        GameState {
            current: GameStateType::default(),
        }
    }
}

pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
    pub obstacle_material: Handle<ColorMaterial>,
    pub enemy_material: Handle<ColorMaterial>,
}
