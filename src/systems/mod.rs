pub mod enemy;
pub mod grid;
pub mod player;
pub mod ranged;
mod shared;
pub mod ui;

use crate::components::player::{Player, PlayerCamera};
use bevy::prelude::*;
use bevy_egui::EguiContext;

const SPRITE_SIZE: f32 = 32.;
const FLOOR_LAYER: f32 = 0.;
const ITEM_LAYER: f32 = 1.;
const MONSTER_LAYER: f32 = 2.;
const PLAYER_LAYER: f32 = 3.;

const MOVE_SIZE: f32 = 32.;

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
    mut egui_context: ResMut<EguiContext>,
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

    // todo: this is only place holder, I think we might need to load it somewhere else
    let face_handle = asset_server.load("placeholders/face.png");
    egui_context.set_egui_texture(1, face_handle);

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
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

use crate::components::Health;
pub fn clear_dead(mut command: Commands, bodies: Query<(Entity, &Health), Without<Player>>) {
    for (entity, hp) in bodies.iter() {
        if hp.current < 0 {
            command.entity(entity).despawn();
        }
    }
}
