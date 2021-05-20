pub mod enemy;
pub mod grid;
pub mod obstacle;
pub mod player;
pub mod ranged;

use crate::components::{PlayerCamera, Position};
use bevy::prelude::*;

const PLAYER_INIT_X: i32 = 1;
const PLAYER_INIT_Y: i32 = 1;

const SPRITE_SIZE: f32 = 32.;
const FLOOR_LAYER: f32 = 0.;
const ITEM_LAYER: f32 = 1.;
const MONSTER_LAYER: f32 = 2.;
const PLAYER_LAYER: f32 = 3.;

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
    let floor_texture = asset_server.get_handle("images/floor.png");
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Position {
            x: PLAYER_INIT_X,
            y: PLAYER_INIT_Y,
        })
        .insert(PlayerCamera);
    commands.insert_resource(super::resources::Materials {
        player_material: materials.add(player_texture.into()),
        obstacle_material: materials.add(wall_texture.into()), //materials.add(Color::rgb(1., 1., 1.).into()),
        enemy_material: materials.add(enemy_texture.into()),
        floor_material: materials.add(floor_texture.into()),
    });
}
