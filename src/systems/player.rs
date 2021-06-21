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
                SystemSet::on_update(GameState::PlayerTurn)
                    .with_system(handle_key_input.system().label(PlayerSystems::HandleInput))
                    .with_system(
                        player_move_or_attack
                            .system()
                            .label(PlayerSystems::PlayerMovement)
                            .after(PlayerSystems::HandleInput),
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
    mut player_action_writer: EventWriter<PlayerActionEvent>,
    player_position: Query<&Transform, With<Player>>,
    blocker_position: Query<(Entity, &Transform, &Blocking)>,
) {
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
                    player_action_writer.send(PlayerActionEvent::Attack(entity))
                }
                Some(_) => (),
                None => player_action_writer.send(PlayerActionEvent::Move(x, y)),
            }
        }
        PlayerAction::RangedTargeting => {
            game_state.set(GameState::RangedTargeting).unwrap();
        }
        PlayerAction::SkipTurn => game_state.set(GameState::EnemyTurn).unwrap(),
        PlayerAction::NoAction => (),
    }
}

pub fn player_move_or_attack(
    mut game_state: ResMut<State<GameState>>,
    mut commands: Commands,
    mut player_action_reader: EventReader<PlayerActionEvent>,
    mut player_camera_pos: QuerySet<(
        Query<&mut Transform, With<Player>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
    mut enemies: Query<(Entity, &mut Health), With<Enemy>>,
    map: Res<crate::systems::grid::Map>,
) {
    match player_action_reader.iter().next() {
        Some(PlayerActionEvent::Move(x, y)) => {
            let mut player_pos = player_camera_pos.q0_mut().single_mut().unwrap();
            //
            if *x < 0. || *y < 0. {
                return;
            }

            if *y >= map.y_size as f32 * super::MOVE_SIZE {
                return;
            };

            if *x >= map.x_size as f32 * super::MOVE_SIZE {
                return;
            };

            // player_pos.update(*x, *y);
            player_pos.translation = Vec3::new(*x, *y, player_pos.translation.z);

            let mut camera_pos = player_camera_pos.q1_mut().single_mut().unwrap();
            camera_pos.translation = Vec3::new(*x, *y, camera_pos.translation.z);
            game_state.set(GameState::EnemyTurn).unwrap();
        }
        Some(PlayerActionEvent::Attack(target)) => {
            if let Some((entity, mut health)) =
                enemies.iter_mut().find(|(entity, _)| entity == target)
            {
                health.current -= 1;
                if health.current <= 0 {
                    commands.entity(entity).despawn();
                }
            };
            game_state.set(GameState::EnemyTurn).unwrap();
        }
        // None => (),
        _ => (),
    };
}
