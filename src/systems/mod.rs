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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    asset_server.load_folder("images").unwrap();
    let wall_texture = asset_server.get_handle("images/wall.png");
    let player_texture = asset_server.get_handle("images/player.png");
    let enemy_texture = asset_server.get_handle("images/enemy.png");
    let floor_texture = asset_server.get_handle("images/floor.png");

    asset_server.load_folder("sprites").unwrap();
    let flamey_handle = asset_server.get_handle("sprites/flamey.png");
    let texture_atlas = TextureAtlas::from_grid(flamey_handle, Vec2::new(32.0, 32.0), 1, 12);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
        flamey_sprite_sheet: texture_atlas_handle,
    });
}

pub fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}
