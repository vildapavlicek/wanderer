use bevy::prelude::*;

pub enum GameState {
    MainMenu,
    PlayerTurn,
    EnemyTurn,
    Targeting,
    Inventory,
}

impl std::default::Default for GameState {
    fn default() -> Self {
        GameState::PlayerTurn
    }
}

pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
    pub obstacle_material: Handle<ColorMaterial>,
    pub enemy_material: Handle<ColorMaterial>,
}
