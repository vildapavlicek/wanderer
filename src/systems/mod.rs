pub mod enemy;
pub mod player;
pub mod ranged;
pub mod ui;

use super::map::SPRITE_SIZE;
use crate::{
    components::{
        player::{Player, PlayerCamera},
        Dead,
    },
    resources::AnimatedSprite,
};
use bevy::prelude::*;
use bevy_egui::EguiContext;

const FLOOR_LAYER: f32 = 0.;
const ITEM_LAYER: f32 = 1.;
const MONSTER_LAYER: f32 = 2.;
const PLAYER_LAYER: f32 = 3.;

const MOVE_SIZE: f32 = SPRITE_SIZE;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SetupSet;

#[derive(Debug, Component)]
pub struct AnimationContext {
    pub first_index: usize,
    pub last_index: usize,
}

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlase_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut egui_context: Query<&EguiContext>,
) {
    let player_texture = asset_server.load("images/player.png");
    let floor_texture = asset_server.load("images/cave_floor_dark.png");
    // let cave_wall_texture = asset_server.load("images/cave_wall4.png");
    let cave_spider = asset_server.load("images/cave_spider.png");
    let mole = asset_server.load("images/mole.png");

    let flamey_handle = asset_server.load("sprites/flamey.png");
    // TODO: This is done in spawn I guess? https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::splat(SPRITE_SIZE), 1, 12, None, None);
    let texture_atlas_handle = texture_atlase_layouts.add(texture_atlas);

    let cave_wall_handle = asset_server.load("sprites/cave_wall3_darker.png");
    // let cave_wall_texture_atlas =
    //     TextureAtlasLayout::from_grid(cave_wall_handle, Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 6, 3);
    // let cave_wall_texture_atlas_handle = texture_atlase_layouts.add(cave_wall_texture_atlas);

    // todo: this is only place holder
    // let face_handle = asset_server.load("placeholders/face.png");
    // egui_context.add_image(face_handle);

    commands
        .spawn(Camera2dBundle::default())
        .insert(PlayerCamera);

    commands.insert_resource(super::resources::Materials {
        cave_spider,
        player_material: player_texture,
        floor_material: floor_texture,
        flamey_sprite_sheet: AnimatedSprite {
            sprite_sheet: flamey_handle,
            atlas_layout: texture_atlas_handle,
            first_index: 1,
            last_index: 11,
        },
        // cave_wall: cave_wall_texture,
        cave_wall_sprite_sheet: cave_wall_handle,
        mole,
    });
}

pub fn animation(
    time: Res<Time>,
    // texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut crate::components::Timer,
        &mut TextureAtlas,
        &AnimationContext,
        // &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut atlas, animation_context) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            if atlas.index == animation_context.last_index {
                atlas.index = animation_context.first_index
            } else {
                atlas.index += 1;
            }
        }
    }
}

pub fn mark_dead(mut command: Commands, health: Query<(Entity, &Health), Without<Player>>) {
    for (entity, health) in health.iter() {
        if health.current <= health.min {
            command.entity(entity).insert(Dead);
        }
    }
}

use crate::components::{Blocking, BlockingType, Health, ItemName};
pub fn clear_dead(mut command: Commands, bodies: Query<Entity, With<Dead>>) {
    bodies
        .iter()
        .for_each(|entity| command.entity(entity).despawn())
}

pub fn cheats(
    mut cmd: Commands,
    mut key_input: ResMut<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut Visibility, &Blocking)>,
) {
    if key_input.just_pressed(KeyCode::KeyH) {
        info!("pressed H");
        for (entity, mut visibility, blocking) in query.iter_mut() {
            if let BlockingType::Wall = blocking.blocking_type {
                *visibility = Visibility::Hidden;
            }
        }
    }

    if key_input.just_pressed(KeyCode::KeyG) {
        info!("pressed G");
        for (entity, mut visibility, blocking) in query.iter_mut() {
            if let BlockingType::Wall = blocking.blocking_type {
                *visibility = Visibility::Visible;
            }
        }
    };
}
