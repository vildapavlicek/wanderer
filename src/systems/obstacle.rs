use crate::components::{Blocking, Obstacle, Position, Size};
use crate::resources::Materials;
use bevy::prelude::*;

pub fn spawn_obstacle(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.obstacle_material.clone(),
            sprite: Sprite::new(Vec2::new(10., 10.)),
            ..Default::default()
        })
        .insert(Obstacle)
        .insert(Blocking)
        .insert(Position { x: 5, y: 5 })
        .insert(Size::square(1.));
}
