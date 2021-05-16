use crate::components::{Blocking, Position, Size};
use crate::resources::Materials;
use bevy::prelude::*;

pub fn spawn_obstacles(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.obstacle_material.clone(),
            sprite: Sprite::new(Vec2::new(10., 10.)),
            ..Default::default()
        })
        .insert(Blocking::obstacle())
        .insert(Position { x: 5, y: 5 })
        .insert(Size::square(1.));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.enemy_material.clone(),
            sprite: Sprite::new(Vec2::new(10., 10.)),
            ..Default::default()
        })
        .insert(Blocking::enemy())
        .insert(Position { x: 6, y: 6 })
        .insert(Size::square(1.));
}
