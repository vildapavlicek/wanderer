use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,
}
pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
    pub obstacle_material: Handle<ColorMaterial>,
    pub enemy_material: Handle<ColorMaterial>,
}
