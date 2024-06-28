use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, States)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,
    RangedTargeting,
}

#[derive(Debug)]
pub struct AnimatedSprite {
    pub sprite_sheet: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub first_index: usize,
    pub last_index: usize,
}

#[derive(Debug, Resource)]
pub struct Materials {
    pub cave_spider: Handle<Image>,
    pub player_material: Handle<Image>,
    pub floor_material: Handle<Image>,
    pub flamey_sprite_sheet: AnimatedSprite,
    pub cave_wall_sprite_sheet: Handle<Image>,
    pub mole: Handle<Image>,
}
