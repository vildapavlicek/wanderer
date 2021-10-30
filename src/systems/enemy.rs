use crate::components::{player::Player, Blocking, Enemy, Health};
use crate::resources::GameState;
// use crate::systems::grid::Map;
use bevy::prelude::*;

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

#[derive(Debug)]
pub enum NPCActionType {
    Move {
        entity: Entity,
        x: f32,
        y: f32,
    },
    /// Attack Action and attacker's details (EntityId, Name)
    Attack(Entity, String),
    RevertDirection(Entity),
}

use crate::components::Name;
pub fn enemy_turn(
    player: Query<(Entity, &Transform), With<Player>>,
    enemies: Query<(Entity, &Transform, &MoveDirection, &Name), With<Enemy>>,
    blockers: Query<(&Transform, &Blocking)>,
) -> Vec<NPCActionType> {
    let mut to_move: Vec<NPCActionType> = vec![];

    let (player_entity, player_pos) = player.single().expect("no player entity");

    for (entity, enemy_pos, move_direction, name) in enemies.iter() {
        if enemy_pos.translation.x + super::MOVE_SIZE == player_pos.translation.x
            && enemy_pos.translation.y == player_pos.translation.y
        {
            to_move.push(NPCActionType::Attack(player_entity, name.to_string()));
            break;
        }

        let future_x = match *move_direction {
            MoveDirection::Left => enemy_pos.translation.x - super::MOVE_SIZE,
            MoveDirection::Right => enemy_pos.translation.x + super::MOVE_SIZE,
        };

        let is_blocked = blockers.iter().any(|(blocker_pos, _)| {
            blocker_pos.translation.x == future_x
                && blocker_pos.translation.y == enemy_pos.translation.y
        });

        if !is_blocked {
            to_move.push(NPCActionType::Move {
                entity,
                x: future_x,
                y: enemy_pos.translation.y,
            })
        } else {
            to_move.push(NPCActionType::RevertDirection(entity))
        }
    }

    to_move
}

use crate::systems::ui::LogEvent;

pub fn enemy_move(
    In(to_move): In<Vec<NPCActionType>>,
    mut q: Query<(&mut Transform, &mut MoveDirection)>,
    mut targets: Query<(Entity, &mut Health), With<Player>>,
    //map: Res<Map>,
    mut game_state: ResMut<State<GameState>>,
    mut log_writer: EventWriter<LogEvent>,
) {
    for action_type in to_move.into_iter() {
        match action_type {
            NPCActionType::Move { entity, x, y } => {
                let (mut position, mut move_direction) = q
                    .get_mut(entity)
                    .expect("requested entity for movement not found");
            }
            NPCActionType::Attack(_target, name) => {
                let (_entity, mut hp) = targets.single_mut().expect("no player entity");
                hp.current -= 1;
                log_writer.send(LogEvent::npc_attacks_player(name, 1));
                info!(msg = "attacked player", ?hp)
            }
            NPCActionType::RevertDirection(entity) => {
                let (_, mut move_direction) = q.get_mut(entity).expect("entity not found");
                *move_direction = move_direction.opposite();
            }
        }
    }
    game_state
        .set(GameState::PlayerTurn)
        .expect("failed to set GameState to PlayerTurn");
}
