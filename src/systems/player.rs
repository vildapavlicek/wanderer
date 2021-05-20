use crate::components::PlayerCamera;
use crate::resources::GameState;
/// Systems related to the player
use crate::{
    components::{Blocking, BlockingType, Enemy, Health, Player, Position, Size},
    resources::Materials,
    systems::PlayerSystems,
};
use bevy::prelude::*;

pub const PLAYER_INIT_MAX_HEALTH: i32 = 100;

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
        .insert(Player)
        .insert(Position {
            x: super::PLAYER_INIT_X,
            y: super::PLAYER_INIT_Y,
        })
        .insert(Size::square(0.8))
        .insert(Health::new(PLAYER_INIT_MAX_HEALTH))
        .insert(Blocking::player());
}

/// This is used to map key to action
#[derive(Debug)]
enum PlayerAction {
    NoAction,
    Movement(i32, i32),
    RangedTargeting,
    SkipTurn,
}

pub enum PlayerActionEvent {
    Move(i32, i32),
    Attack(Entity),
}

pub fn handle_key_input(
    mut game_state: ResMut<State<GameState>>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut player_action_writer: EventWriter<PlayerActionEvent>,
    player_position: Query<&Position, With<Player>>,
    blocker_position: Query<(Entity, &Position, &Blocking)>,
) {
    let player_position = player_position.single().expect("no player position!!");

    let mut action = if key_input.just_pressed(KeyCode::Left) {
        PlayerAction::Movement(player_position.x - 1, player_position.y)
    } else if key_input.just_pressed(KeyCode::Up) {
        PlayerAction::Movement(player_position.x, player_position.y + 1)
    } else if key_input.just_pressed(KeyCode::Right) {
        PlayerAction::Movement(player_position.x + 1, player_position.y)
    } else if key_input.just_pressed(KeyCode::Down) {
        PlayerAction::Movement(player_position.x, player_position.y - 1)
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
            match blocker_position
                .iter()
                .find(|(entity, pos, blocking)| (pos.x == x) && (pos.y == y))
            {
                Some((entity, _, blocking)) if blocking.is_attackable() => {
                    player_action_writer.send(PlayerActionEvent::Attack(entity))
                }
                Some(_) => (),
                None => player_action_writer.send(PlayerActionEvent::Move(x, y)),
            }
        }
        PlayerAction::RangedTargeting => {
            info!(msg = "switching to RangedTargeting state");
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
        Query<&mut Position, With<Player>>,
        Query<&mut Position, With<PlayerCamera>>,
    )>,
    mut enemies: Query<(Entity, &mut Health), With<Enemy>>,
    map: Res<crate::systems::grid::Map>,
) {
    match player_action_reader.iter().next() {
        Some(PlayerActionEvent::Move(x, y)) => {
            let mut player_pos = player_camera_pos.q0_mut().single_mut().unwrap();

            if (*x < 0 || *y < 0) {
                return;
            }

            if *y >= map.y_size as i32 {
                return;
            };

            if *x >= map.x_size as i32 {
                return;
            };

            player_pos.update(*x, *y);

            let mut camera_pos = player_camera_pos.q1_mut().single_mut().unwrap();
            camera_pos.update(*x, *y);
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
        None => (),
    };
}
