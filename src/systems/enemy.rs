use crate::components::{Blocking, Enemy, Health, Player};
use crate::resources::GameState;
use crate::systems::grid::Map;
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

pub enum MoveDirection {
    Left,
    Right,
}

impl MoveDirection {
    pub fn opposite(&self) -> Self {
        match self {
            MoveDirection::Left => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Left,
        }
    }
}

impl std::default::Default for MoveDirection {
    fn default() -> Self {
        MoveDirection::Left
    }
}

struct NPCAction {}

#[derive(Debug)]
pub enum NPCActionType {
    Move { entity: Entity, x: f32, y: f32 },
    Attack(Entity),
    RevertDirection(Entity),
}

pub fn enemy_turn(
    player: Query<(Entity, &Transform), With<Player>>,
    movers: Query<(Entity, &Transform, &MoveDirection), With<Enemy>>,
    blockers: Query<(&Transform, &Blocking)>,
) -> Vec<NPCActionType> {
    let mut to_move: Vec<NPCActionType> = vec![];

    let (player_entity, player_pos) = player.single().expect("no player entity");

    for (entity, mover_pos, move_direction) in movers.iter() {
        if mover_pos.translation.x + super::MOVE_SIZE == player_pos.translation.x
            && mover_pos.translation.y == player_pos.translation.y
        {
            to_move.push(NPCActionType::Attack(player_entity));
            break;
        }

        let future_x = match *move_direction {
            MoveDirection::Left => mover_pos.translation.x - super::MOVE_SIZE,
            MoveDirection::Right => mover_pos.translation.x + super::MOVE_SIZE,
        };

        let is_blocked = blockers
            .iter()
            .find(|(blocker_pos, _)| {
                blocker_pos.translation.x == future_x
                    && blocker_pos.translation.y == mover_pos.translation.y
            })
            .is_some();

        if !is_blocked {
            to_move.push(NPCActionType::Move {
                entity,
                x: future_x,
                y: mover_pos.translation.y,
            })
        } else {
            to_move.push(NPCActionType::RevertDirection(entity))
        }
    }

    to_move
}

pub fn enemy_move(
    In(to_move): In<Vec<NPCActionType>>,
    mut q: Query<(&mut Transform, &mut MoveDirection)>,
    mut targets: Query<(Entity, &mut Health), With<Player>>,
    map: Res<Map>,
    mut game_state: ResMut<State<GameState>>,
) {
    for action_type in to_move.iter() {
        match action_type {
            NPCActionType::Move { entity, x, y } => {
                let (mut position, mut move_direction) = q
                    .get_mut(*entity)
                    .expect("requested entity for movement not found");

                if is_out_of_bounds(*x, *y, map.x_size as i32, map.y_size as i32) {
                    *move_direction = move_direction.opposite();
                } else {
                    position.translation.x = *x;
                }
            }
            NPCActionType::Attack(_target) => {
                let (_entity, mut hp) = targets.single_mut().expect("no player entity");
                hp.current -= 1;
                info!(msg = "attacked player", ?hp)
            }
            NPCActionType::RevertDirection(entity) => {
                let (_, mut move_direction) = q.get_mut(*entity).expect("entity not found");
                *move_direction = move_direction.opposite();
            }
        }
    }
    game_state.set(GameState::PlayerTurn);
}

fn is_out_of_bounds(x: f32, y: f32, max_x: i32, max_y: i32) -> bool {
    x < 0. || x >= (max_x * 32) as f32 || y < 0. || y >= (max_y * 32) as f32
}
