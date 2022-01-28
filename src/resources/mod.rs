use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,
    RangedTargeting,
}
pub struct Materials {
    pub cave_spider: Handle<Image>,
    pub player_material: Handle<Image>,
    pub player24x24_material: Handle<Image>,
    pub obstacle_material: Handle<Image>,
    pub enemy_material: Handle<Image>,
    pub floor_material: Handle<Image>,
    pub flamey_sprite_sheet: Handle<TextureAtlas>,
    pub cave_wall: Handle<Image>,
    pub cave_wall_sprite_sheet: Handle<TextureAtlas>,
    pub mole: Handle<Image>,
}
