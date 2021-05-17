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
    mut game_state: ResMut<GameState>,
    mut move_direction: Local<MoveDirection>,
    mut query: Query<(&mut Position), With<Enemy>>,
) {
    if !game_state.is_enemy_turn() {
        return;
    }

    for mut position in query.iter_mut() {
        match move_direction.deref() {
            MoveDirection::Left => {
                position.x -= 1;
                if position.x <= 0 {
                    *move_direction = MoveDirection::Right
                }
            }
            MoveDirection::Right => {
                position.x += 1;
                if position.x >= super::grid::ARENA_WIDTH as i32 - 1 {
                    *move_direction = MoveDirection::Left
                }
            }
        }
    }

    game_state.next();
}
