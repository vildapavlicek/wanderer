pub mod enemy;
pub mod grid;
pub mod obstacle;
pub mod player;
pub mod ranged;

use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystems {
    HandleInput,
    PlayerMovement,
}

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    asset_server.load_folder("images").unwrap();
    let wall_texture = asset_server.get_handle("images/wall.png");
    let player_texture = asset_server.get_handle("images/player.png");
    let enemy_texture = asset_server.get_handle("images/enemy.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(super::resources::Materials {
        player_material: materials.add(player_texture.into()),
        obstacle_material: materials.add(wall_texture.into()), //materials.add(Color::rgb(1., 1., 1.).into()),
        enemy_material: materials.add(enemy_texture.into()),
    });
}
