use crate::components::{Blocking, BlockingType, Player, Position, Size};
use crate::resources::Materials;
use crate::systems::PlayerSystems;
use bevy::input::keyboard::KeyCode::Key0;
/// Systems related to the player
use bevy::prelude::*;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("spawn_player", SystemStage::single(spawn_player.system()))
            .add_system_set(
                SystemSet::new()
                    .with_system(handle_key_input.system().label(PlayerSystems::HandleInput))
                    .with_system(
                        player_movement
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
            sprite: Sprite::new(Vec2::new(10., 10.)),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        })
        .insert(Player)
        .insert(Position { x: 1, y: 1 })
        .insert(Size::square(0.8));
}

/// This is used to map key to action
enum PlayerAction {
    NoAction,
    Movement(i32, i32),
}

pub enum PlayerActionEvent {
    Move(i32, i32),
    Attack(Entity),
}

pub fn handle_key_input(
    key_input: Res<Input<KeyCode>>,
    mut player_action_writer: EventWriter<PlayerActionEvent>,
    player_position: Query<&Position, With<Player>>,
    blocker_position: Query<(Entity, &Position, &Blocking)>,
) {
    let player_position = player_position.single().expect("no player position!!");

    let action = if key_input.just_pressed(KeyCode::Left) {
        PlayerAction::Movement(player_position.x - 1, player_position.y)
    } else if key_input.just_pressed(KeyCode::Up) {
        PlayerAction::Movement(player_position.x, player_position.y + 1)
    } else if key_input.just_pressed(KeyCode::Right) {
        PlayerAction::Movement(player_position.x + 1, player_position.y)
    } else if key_input.just_pressed(KeyCode::Down) {
        PlayerAction::Movement(player_position.x, player_position.y - 1)
    } else {
        PlayerAction::NoAction
    };

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
        PlayerAction::NoAction => (),
    }
}

pub fn player_movement(
    mut player_action_reader: EventReader<PlayerActionEvent>,
    mut player_position: Query<&mut Position, With<Player>>,
) {
    match player_action_reader.iter().next() {
        Some(PlayerActionEvent::Move(x, y)) => {
            let mut pos = player_position.single_mut().unwrap();
            pos.x = *x;
            pos.y = *y;
        }
        Some(PlayerActionEvent::Attack(_)) => (),
        None => (),
    }
}
