use crate::components::{Blocking, Obstacle, Player, Position, Size};
use crate::events::player::PlayerActionEvent;
use crate::resources::Materials;
/// Systems related to the player
use bevy::prelude::*;

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

pub fn handle_key_input(
    key_input: Res<Input<KeyCode>>,
    mut player_action_writer: EventWriter<PlayerActionEvent>,
    player_position: Query<&Position, With<Player>>,
    blocker_position: Query<&Position, With<Blocking>>,
) {
    let player_position = player_position.iter().next().expect("no player position!!");

    if key_input.just_pressed(KeyCode::Left) {
        if let Some(_blocked) = blocker_position.iter().find(|pos| {
            info!(
                x = format!("{}", pos.x).as_str(),
                px = format!("{} - 1", player_position.x).as_str()
            );
            (pos.x == (player_position.x - 1)) && (pos.y == player_position.y)
        }) {
            info!(
                msg = "position blocked, cannot move",
                x = _blocked.x,
                y = _blocked.y
            );
            return;
        }
        player_action_writer.send(PlayerActionEvent::MoveLeft)
    }

    if key_input.just_pressed(KeyCode::Up) {
        if let Some(_blocked) = blocker_position.iter().find(|pos| {
            info!(
                y = format!("{}", pos.y).as_str(),
                py = format!("{} + 1", player_position.y).as_str(),
            );
            (pos.y == (player_position.y + 1)) && (pos.x == player_position.x)
        }) {
            info!(
                msg = "position blocked, cannot move",
                x = _blocked.x,
                y = _blocked.y
            );
            return;
        }
        player_action_writer.send(PlayerActionEvent::MoveUp)
    }

    if key_input.just_pressed(KeyCode::Right) {
        if let Some(_blocked) = blocker_position.iter().find(|pos| {
            info!(
                x = format!("{}", pos.x).as_str(),
                px = format!("{} + 1", player_position.x).as_str()
            );
            pos.x == (player_position.x + 1) && (pos.y == player_position.y)
        }) {
            info!(
                msg = "position blocked, cannot move",
                x = _blocked.x,
                y = _blocked.y
            );
            return;
        }
        player_action_writer.send(PlayerActionEvent::MoveRight)
    }

    if key_input.just_pressed(KeyCode::Down) {
        if let Some(_blocked) = blocker_position.iter().find(|pos| {
            info!(
                y = format!("{}", pos.y).as_str(),
                py = format!("{} - 1", player_position.y).as_str()
            );
            pos.y == (player_position.y - 1) && (pos.x == player_position.x)
        }) {
            info!(
                msg = "position blocked, cannot move",
                x = _blocked.x,
                y = _blocked.y
            );
            return;
        }
        player_action_writer.send(PlayerActionEvent::MoveDown)
    }
}

pub fn player_movement(
    mut player_action_reader: EventReader<PlayerActionEvent>,
    mut player_position: Query<&mut Position, With<Player>>,
) {
    if let Some(player_action) = player_action_reader.iter().next() {
        if let Some(mut player_position) = player_position.iter_mut().next() {
            info!(
                msg = "position before movement",
                x = player_position.x,
                y = player_position.y
            );
            match player_action {
                PlayerActionEvent::MoveLeft => player_position.x -= 1,
                PlayerActionEvent::MoveUp => player_position.y += 1,
                PlayerActionEvent::MoveRight => player_position.x += 1,
                PlayerActionEvent::MoveDown => player_position.y -= 1,
                _ => (),
            }
            info!(
                msg = "moved to position",
                x = player_position.x,
                y = player_position.y
            )
        }
    }
}
