use crate::components::{Enemy, Player, Position};
use crate::resources::GameState;
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

pub enum MoveDirection {
    Left,
    Right,
}

impl std::default::Default for MoveDirection {
    fn default() -> Self {
        MoveDirection::Left
    }
}

pub fn enemy_move(
    mut game_state: ResMut<State<GameState>>,
    mut move_direction: Local<MoveDirection>,
    mut query: QuerySet<(
        Query<&Position, With<Player>>,
        Query<&mut Position, With<Enemy>>,
    )>,
) {
    let player_position = query.q0().single().expect("player position not found");
    let (player_x, player_y) = (player_position.x, player_position.y);

    for mut position in query.q1_mut().iter_mut() {
        let future_x = match move_direction.deref() {
            MoveDirection::Left => {
                let x = position.x - 1;
                x
            }
            MoveDirection::Right => {
                let x = position.x + 1;
                x
            }
        };

        if !(future_x == player_x && player_y == position.y) {
            position.x = future_x;

            match move_direction.deref() {
                MoveDirection::Left if position.x <= 0 => *move_direction = MoveDirection::Right,
                MoveDirection::Right if position.x >= super::grid::ARENA_WIDTH as i32 - 1 => {
                    *move_direction = MoveDirection::Left
                }
                _ => (),
            }
        }
    }

    game_state.set(GameState::PlayerTurn).unwrap();
}
