pub mod player;
pub mod sandbox;
/* pub mod enemy;

pub mod ranged;
pub mod ui;



use bevy_egui::EguiContext;
*/

const SPRITE_SIZE: f32 = 32.;
const FLOOR_LAYER: f32 = 0.;
const ITEM_LAYER: f32 = 1.;
const MONSTER_LAYER: f32 = 2.;
const PLAYER_LAYER: f32 = 3.;

const MOVE_SIZE: f32 = 32.;
/*
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystems {
    HandleInput,
    PlayerMovement,
}
*/

use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut egui_context: ResMut<EguiContext>,
) {
    asset_server.load_folder("images").unwrap();
    let wall_texture = asset_server.get_handle("images/wall.png");
    let player_texture = asset_server.get_handle("images/player.png");
    let player_texture_24x24 = asset_server.get_handle("images/player24x24.png");
    let enemy_texture = asset_server.get_handle("images/enemy.png");
    let floor_texture = asset_server.get_handle("images/cave_floor_dark.png");
    let cave_wall_texture = asset_server.get_handle("images/cave_wall4.png");
    let cave_spider = asset_server.get_handle("images/cave_spider.png");
    let mole = asset_server.get_handle("images/mole.png");

    asset_server.load_folder("sprites").unwrap();
    let flamey_handle = asset_server.get_handle("sprites/flamey.png");
    let texture_atlas = TextureAtlas::from_grid(flamey_handle, Vec2::new(32.0, 32.0), 1, 12);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let cave_wall_handle = asset_server.get_handle("sprites/cave_wall3_darker.png");
    let cave_wall_texture_atlas =
        TextureAtlas::from_grid(cave_wall_handle, Vec2::new(32., 32.), 6, 3);
    let cave_wall_texture_atlas_handle = texture_atlases.add(cave_wall_texture_atlas);

    // todo: this is only place holder
    /* let face_handle = asset_server.load("placeholders/face.png");
    egui_context.add_image(face_handle); */

    commands.insert_resource(super::resources::Materials {
        cave_spider,
        player_material: player_texture,
        obstacle_material: wall_texture,
        enemy_material: enemy_texture,
        floor_material: floor_texture,
        player24x24_material: player_texture_24x24,
        flamey_sprite_sheet: texture_atlas_handle,
        cave_wall: cave_wall_texture,
        cave_wall_sprite_sheet: cave_wall_texture_atlas_handle,
        mole,
    });
}
/*
pub fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut crate::components::Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len());
        }
    }
}

/* use crate::components::{Blocking, BlockingType, Health, ItemName};
pub fn clear_dead(mut command: Commands, bodies: Query<(Entity, &Health), Without<Player>>) {
    for (entity, hp) in bodies.iter() {
        if hp.current < 0 {
            command.entity(entity).despawn();
        }
    }
} */

/* pub fn cheats(
    mut cmd: Commands,
    mut key_input: ResMut<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Visibility, &Blocking)>,
) {
    if key_input.just_pressed(KeyCode::H) {
        info!("pressed H");
        for (entity, mut visibility, blocking) in query.iter_mut() {
            if let BlockingType::Wall = blocking.blocking_type {
                visibility.is_visible = false;
            }
        }
    }

    if key_input.just_pressed(KeyCode::G) {
        info!("pressed G");
        for (entity, mut visibility, blocking) in query.iter_mut() {
            if let BlockingType::Wall = blocking.blocking_type {
                visibility.is_visible = true;
            }
        }
    };
} */
 */
