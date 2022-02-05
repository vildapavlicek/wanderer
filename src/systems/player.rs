/// Systems related to the player
use crate::{
    components::{
        player::{Player, PlayerBundle, PlayerCamera},
        Blocking, Enemy, Health,
    },
    resources::{GameState, Materials},
    systems::PlayerSystems,
};
use bevy::prelude::*;
use std::default::Default;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("spawn_player", SystemStage::single(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::PlayerTurn).with_system(
                    handle_key_input
                        .system()
                        .chain(player_move_or_attack.system())
                        .label(PlayerSystems::HandleInput),
                ),
            )
            .add_event::<PlayerActionEvent>();
    }
}

pub fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: materials.player_material.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., super::PLAYER_LAYER),
            ..Default::default()
        })
        .insert_bundle(PlayerBundle::new(10));
}

/// This is used to map key to action
#[derive(Debug)]
enum PlayerAction {
    NoAction,
    Movement(f32, f32),
    RangedTargeting,
    SkipTurn,
}

pub enum PlayerActionEvent {
    Move(f32, f32),
    Attack(Entity),
}

pub fn handle_key_input(
    mut game_state: ResMut<State<GameState>>,
    mut key_input: ResMut<Input<KeyCode>>,
    player_position: Query<&Transform, With<Player>>,
    blocker_position: Query<(Entity, &Transform, &Blocking)>,
) -> Option<PlayerActionEvent> {
    let player_position = player_position.single();

    let action = if key_input.just_pressed(KeyCode::Left) {
        PlayerAction::Movement(
            player_position.translation.x - super::MOVE_SIZE,
            player_position.translation.y,
        )
    } else if key_input.just_pressed(KeyCode::Up) {
        PlayerAction::Movement(
            player_position.translation.x,
            player_position.translation.y + super::MOVE_SIZE,
        )
    } else if key_input.just_pressed(KeyCode::Right) {
        PlayerAction::Movement(
            player_position.translation.x + super::MOVE_SIZE,
            player_position.translation.y,
        )
    } else if key_input.just_pressed(KeyCode::Down) {
        PlayerAction::Movement(
            player_position.translation.x,
            player_position.translation.y - super::MOVE_SIZE,
        )
    } else if key_input.just_pressed(KeyCode::T) {
        PlayerAction::RangedTargeting
    } else if key_input.just_pressed(KeyCode::S) {
        PlayerAction::SkipTurn
    } else {
        PlayerAction::NoAction
    };

    // need to update or the last key input gets cached and freezes app
    key_input.clear();

    match action {
        PlayerAction::Movement(x, y) => {
            match blocker_position.iter().find(|(_, blocker_pos, _)| {
                (blocker_pos.translation.x == x) && (blocker_pos.translation.y == y)
            }) {
                Some((entity, _, blocking)) if blocking.is_attackable() => {
                    Some(PlayerActionEvent::Attack(entity))
                }
                Some(_) => None,
                None => Some(PlayerActionEvent::Move(x, y)), // player_action_writer.send(PlayerActionEvent::Move(x, y)),
            }
        }
        PlayerAction::RangedTargeting => {
            game_state
                .set(GameState::RangedTargeting)
                .expect("failed to change game state to RangedTargeting");
            None
        }
        PlayerAction::SkipTurn => {
            game_state
                .set(GameState::EnemyTurn)
                .expect("failed to set enemy turn after player skipping turn");
            None
        }
        PlayerAction::NoAction => None,
    }
}

use crate::systems::ui::LogEvent;

pub fn player_move_or_attack(
    In(event): In<Option<PlayerActionEvent>>,
    mut game_state: ResMut<State<GameState>>,
    mut cameras: Query<&mut Transform, Or<(With<Player>, With<PlayerCamera>)>>,
    mut enemies: Query<(Entity, &mut Health, &crate::components::ItemName), With<Enemy>>,
    mut log_writer: EventWriter<LogEvent>,
) {
    match event {
        Some(PlayerActionEvent::Move(x, y)) => {
            cameras
                .iter_mut()
                .for_each(|mut t| t.translation = Vec3::new(x, y, t.translation.z));

            game_state
                .set(GameState::EnemyTurn)
                .expect("failed to set game state to enemy turn after player movement");
        }
        Some(PlayerActionEvent::Attack(target)) => {
            if let Ok((_, mut health, name)) = enemies.get_mut(target) {
                health.current -= 1;
                log_writer.send(LogEvent::player_attack(name.to_string(), 1));
            }

            game_state
                .set(GameState::EnemyTurn)
                .expect("failed to set game state to enemy turn after player attack");
        }
        _ => (),
    };
}
