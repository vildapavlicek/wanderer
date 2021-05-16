pub mod grid;
pub mod obstacle;
pub mod player;

use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystems {
    HandleInput,
    PlayerMovement,
}

pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(super::resources::Materials {
        player_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        obstacle_material: materials.add(Color::rgb(1., 1., 1.).into()),
        enemy_material: materials.add(Color::rgb(1., 0., 0.).into()),
    });
}
