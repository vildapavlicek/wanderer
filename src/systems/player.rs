/// Systems related to the player
use crate::{
    components::{
        player::{Player, PlayerBundle, PlayerCamera},
        Blocking, Enemy, Health,
    },
    map::MapGenSet,
    resources::{GameState, Materials},
};
use bevy::prelude::*;
use std::default::Default;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerSetupSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerTurnSet;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            spawn_player.run_if(run_once()).in_set(PlayerSetupSet),
        )
        .add_systems(OnEnter(GameState::PlayerTurn), check_player_health)
        .add_systems(
            Update,
            (handle_key_input.pipe(player_move_or_attack))
                .run_if(in_state(GameState::PlayerTurn))
                .in_set(PlayerTurnSet),
        )
        .configure_sets(Startup, PlayerSetupSet.after(MapGenSet))
        .configure_sets(Update, PlayerTurnSet.run_if(player_spawned));
    }
}

pub fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn((
        SpriteBundle {
            texture: materials.player_material.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., super::PLAYER_LAYER),
            ..Default::default()
        },
        PlayerBundle::new(10),
    ));
}

pub fn player_spawned(player: Query<&Player>) -> bool {
    player.get_single().is_ok()
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
    mut game_state: ResMut<NextState<GameState>>,
    mut key_input: ResMut<ButtonInput<KeyCode>>,
    player_position: Query<&Transform, With<Player>>,
    blocker_position: Query<(Entity, &Transform, &Blocking)>,
) -> Option<PlayerActionEvent> {
    let player_position = player_position.single();

    let action = if key_input.just_pressed(KeyCode::ArrowLeft) {
        PlayerAction::Movement(
            player_position.translation.x - super::MOVE_SIZE,
            player_position.translation.y,
        )
    } else if key_input.just_pressed(KeyCode::ArrowUp) {
        PlayerAction::Movement(
            player_position.translation.x,
            player_position.translation.y + super::MOVE_SIZE,
        )
    } else if key_input.just_pressed(KeyCode::ArrowRight) {
        PlayerAction::Movement(
            player_position.translation.x + super::MOVE_SIZE,
            player_position.translation.y,
        )
    } else if key_input.just_pressed(KeyCode::ArrowDown) {
        PlayerAction::Movement(
            player_position.translation.x,
            player_position.translation.y - super::MOVE_SIZE,
        )
    } else if key_input.just_pressed(KeyCode::KeyT) {
        PlayerAction::RangedTargeting
    } else if key_input.just_pressed(KeyCode::KeyS) {
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
            game_state.set(GameState::RangedTargeting);
            None
        }
        PlayerAction::SkipTurn => {
            game_state.set(GameState::EnemyTurn);
            None
        }
        PlayerAction::NoAction => None,
    }
}

use crate::systems::ui::LogEvent;

pub fn player_move_or_attack(
    In(event): In<Option<PlayerActionEvent>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut cameras: Query<&mut Transform, Or<(With<Player>, With<PlayerCamera>)>>,
    mut enemies: Query<(Entity, &mut Health, &crate::components::ItemName), With<Enemy>>,
    mut log_writer: EventWriter<LogEvent>,
) {
    match event {
        Some(PlayerActionEvent::Move(x, y)) => {
            cameras
                .iter_mut()
                .for_each(|mut t| t.translation = Vec3::new(x, y, t.translation.z));

            game_state.set(GameState::EnemyTurn);
        }
        Some(PlayerActionEvent::Attack(target)) => {
            if let Ok((_, mut health, name)) = enemies.get_mut(target) {
                health.current -= 1;
                log_writer.send(LogEvent::player_attack(name.to_string(), 1));
            }

            game_state.set(GameState::EnemyTurn);
        }
        _ => (),
    };
}

pub fn check_player_health(
    query: Query<&Health, With<Player>>,
    mut state: ResMut<NextState<GameState>>,
) {
    let health = query.single();

    if health.current <= health.min {
        state.set(GameState::PlayerDead);
        warn!("player died!!!!!!");
    }
}
