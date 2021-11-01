use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,
    RangedTargeting,
}
pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
    pub player24x24_material: Handle<ColorMaterial>,
    pub obstacle_material: Handle<ColorMaterial>,
    pub enemy_material: Handle<ColorMaterial>,
    pub floor_material: Handle<ColorMaterial>,
    pub flamey_sprite_sheet: Handle<TextureAtlas>,
    pub cave_wall_sprite_sheet: Handle<TextureAtlas>,
    pub cave_wall: Handle<ColorMaterial>,
}
