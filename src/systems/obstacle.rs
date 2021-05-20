use crate::components::{Blocking, Enemy, Health, Position, Size};
use crate::resources::Materials;
use bevy::prelude::*;

// pub fn spawn_obstacles(mut commands: Commands, materials: Res<Materials>) {
//     // wall
//     commands
//         .spawn_bundle(SpriteBundle {
//             material: materials.obstacle_material.clone(),
//             sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
//             transform: Transform::from_xyz(0., 0., super::MONSTER_LAYER),
//             ..Default::default()
//         })
//         .insert(Blocking::obstacle())
//         .insert(Position { x: 5, y: 5 })
//         .insert(Size::square(1.));
//
//     // our enemy
//     commands
//         .spawn_bundle(SpriteBundle {
//             material: materials.enemy_material.clone(),
//             sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
//             transform: Transform::from_xyz(0., 0., super::MONSTER_LAYER),
//             ..Default::default()
//         })
//         .insert(Blocking::enemy())
//         .insert(Position { x: 6, y: 6 })
//         .insert(Size::square(1.))
//         .insert(Health::new(50))
//         .insert(Enemy);
// }
