use bevy::prelude::*;

pub struct Materials {
    pub player_material: Handle<ColorMaterial>,
    pub obstacle_material: Handle<ColorMaterial>,
    pub enemy_material: Handle<ColorMaterial>,
}
