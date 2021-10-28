use crate::resources::GameState;
/// Systems related to the player
use crate::{
    components::{
        player::{Player, PlayerBundle, PlayerCamera},
        Blocking, Enemy, Health,
    },
    resources::Materials,
    systems::PlayerSystems,
};
use bevy::prelude::*;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("spawn_player", SystemStage::single(spawn_player.system()))
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
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
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
    let player_position = player_position.single().expect("no player position!!");

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
    key_input.update();

    match action {
        PlayerAction::Movement(x, y) => {
            match blocker_position.iter().find(|(_, blocker_pos, _)| {
                (blocker_pos.translation.x == x) && (blocker_pos.translation.y == y)
            }) {
                Some((entity, _, blocking)) if blocking.is_attackable() => {
                    // player_action_writer.send(PlayerActionEvent::Attack(entity))
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
    mut player_camera_pos: QuerySet<(
        Query<&mut Transform, With<Player>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
    mut enemies: Query<(Entity, &mut Health, &crate::components::Name), With<Enemy>>,
    // map: Res<crate::systems::grid::Map>,
    mut log_writer: EventWriter<LogEvent>,
) {
    match event {
        Some(PlayerActionEvent::Move(x, y)) => {
            /* if super::shared::is_out_of_bounds(x, y, map.x_size, map.y_size) {
                return;
            } */

            let mut player_pos = player_camera_pos
                .q0_mut()
                .single_mut()
                .expect("no player found");

            // player_pos.update(*x, *y);
            player_pos.translation = Vec3::new(x, y, player_pos.translation.z);

            let mut camera_pos = player_camera_pos
                .q1_mut()
                .single_mut()
                .expect("no player camera found");
            camera_pos.translation = Vec3::new(x, y, camera_pos.translation.z);

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
