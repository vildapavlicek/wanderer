use crate::ai::actions::{Idle, Move};
use crate::components::{player::Player, Blocking, Enemy, Health};
use crate::map::SPRITE_SIZE;
use crate::resources::GameState;
use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use big_brain::actions::ActionState;
use big_brain::prelude::Actor;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RandomMoveDirection {
    Up,
    Right,
    Down,
    Left,
}

impl Distribution<RandomMoveDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RandomMoveDirection {
        match rng.gen_range(0..=3) {
            0 => RandomMoveDirection::Up,
            1 => RandomMoveDirection::Right,
            2 => RandomMoveDirection::Down,
            3 => RandomMoveDirection::Left,
            _ => panic!("reached unexpected value"),
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyTurnSet;

#[derive(Debug)]
pub enum NPCActionType {
    Move {
        actor: Entity,
        new_position: Vec3,
    },
    /// Attack Action and attacker's details (EntityId, Name)
    Attack {
        target: Entity,
        attacker_name: String,
    },
}

use crate::components::{Dead, ItemName};

pub fn enemy_turn(
    player: Query<(Entity, &Transform), With<Player>>,
    enemies: Query<(Entity, &Transform, &ItemName), (With<Enemy>, Without<Dead>)>,
    mut actors: Query<(&Actor, &mut ActionState), (With<Move>, Without<Idle>)>,
    mut idle_actors: Query<(&Actor, &mut ActionState), (With<Idle>, Without<Move>)>,
    blockers: Query<(&Transform, &Blocking), Without<Player>>,
) -> Vec<NPCActionType> {
    let mut to_move: Vec<NPCActionType> = vec![];

    let (player_entity, player_pos) = player.single();
    // create hashset of occupied possitions, ie positions that the enemy cannot move to
    let mut occupied = HashSet::<I64Vec2>::from_iter(
        blockers
            .iter()
            .map(|(transform, _)| transform.translation.truncate().as_i64vec2()),
    );

    for (Actor(actor), mut action_state) in actors.iter_mut() {
        if !matches!(*action_state, ActionState::Requested) {
            continue;
        }

        let Ok((entity, npc_transform, name)) = enemies.get(*actor) else {
            continue;
        };

        let Some(future_pos) = resolve_position(npc_transform, player_pos, &occupied) else {
            // in this case there is nowhere to move, so we just mark the action as success and move on
            *action_state = big_brain::actions::ActionState::Success;
            continue;
        };

        // if our new position is the same as the player position, then instead of moving, we attack the player
        if future_pos.truncate() == player_pos.translation.truncate() {
            to_move.push(NPCActionType::Attack {
                target: player_entity,
                attacker_name: name.to_string(),
            });

            *action_state = big_brain::actions::ActionState::Success;
            continue;
        }

        to_move.push(NPCActionType::Move {
            actor: entity,
            new_position: future_pos,
        });

        // if we move to new position, we also have to add it into the set of occupied positions
        occupied.insert(future_pos.truncate().as_i64vec2());

        *action_state = big_brain::actions::ActionState::Success;
    }

    for (Actor(entity), mut action_state) in idle_actors.iter_mut() {
        if !matches!(*action_state, ActionState::Requested) {
            continue;
        }

        let Ok((_, transform, name)) = enemies.get(*entity) else {
            continue;
        };

        let mut possible_tries = vec![
            RandomMoveDirection::Up,
            RandomMoveDirection::Right,
            RandomMoveDirection::Down,
            RandomMoveDirection::Left,
        ];

        while !possible_tries.is_empty() {
            let move_direction: RandomMoveDirection = rand::random();
            if !possible_tries.contains(&move_direction) {
                continue;
            }
            possible_tries
                .iter()
                .position(|v| *v == move_direction)
                .map(|index| possible_tries.remove(index));

            let new_pos = Vec3::new(
                ((matches!(move_direction, RandomMoveDirection::Right)) as i8
                    - (matches!(move_direction, RandomMoveDirection::Left)) as i8)
                    as f32
                    * SPRITE_SIZE,
                ((matches!(move_direction, RandomMoveDirection::Up)) as i8
                    - (matches!(move_direction, RandomMoveDirection::Down)) as i8)
                    as f32
                    * SPRITE_SIZE,
                0f32,
            );

            let new_pos = transform.translation + new_pos;

            // if the new position is occupied we just move on
            if occupied.contains(&new_pos.truncate().as_i64vec2()) {
                continue;
            }

            to_move.push(NPCActionType::Move {
                actor: *entity,
                new_position: new_pos,
            });

            occupied.insert(new_pos.truncate().as_i64vec2());
            break;
        }

        *action_state = ActionState::Success;
    }

    to_move
}

/// Here we resolve in what direction the enemy should move in to get close to the player, ie down, up, left or right.
/// If correctly resolved and the position we should move to is not occupied, we then we return new position
fn resolve_position(
    npc: &Transform,
    player: &Transform,
    blockers: &HashSet<I64Vec2>,
) -> Option<Vec3> {
    // if player is right to the npc
    if player.translation.x > npc.translation.x
        && !blockers.contains(&I64Vec2::new(
            (npc.translation.x + super::SPRITE_SIZE) as i64,
            npc.translation.y as i64,
        ))
    {
        return Some(Vec3::new(
            npc.translation.x + super::SPRITE_SIZE,
            npc.translation.y,
            npc.translation.z,
        ));
    }

    // if player is left to the npc
    if player.translation.x < npc.translation.x
        && !blockers.contains(&I64Vec2::new(
            (npc.translation.x - super::SPRITE_SIZE) as i64,
            npc.translation.y as i64,
        ))
    {
        return Some(Vec3::new(
            npc.translation.x - super::SPRITE_SIZE,
            npc.translation.y,
            npc.translation.z,
        ));
    }

    // if player is above the npc
    if player.translation.y > npc.translation.y
        && !blockers.contains(&I64Vec2::new(
            npc.translation.x as i64,
            (npc.translation.y + super::SPRITE_SIZE) as i64,
        ))
    {
        return Some(Vec3::new(
            npc.translation.x,
            npc.translation.y + super::SPRITE_SIZE,
            npc.translation.z,
        ));
    }

    // if player is bellow the npc
    if player.translation.y < npc.translation.y
        && !blockers.contains(&I64Vec2::new(
            npc.translation.x as i64,
            (npc.translation.y - super::SPRITE_SIZE) as i64,
        ))
    {
        return Some(Vec3::new(
            npc.translation.x,
            npc.translation.y - super::SPRITE_SIZE,
            npc.translation.z,
        ));
    }

    None
}

use crate::systems::ui::LogEvent;

pub fn enemy_move(
    In(to_move): In<Vec<NPCActionType>>,
    mut q: Query<&mut Transform>,
    mut targets: Query<(Entity, &mut Health)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut log_writer: EventWriter<LogEvent>,
) {
    for action_type in to_move.into_iter() {
        match action_type {
            NPCActionType::Move {
                actor: entity,
                new_position,
            } => {
                let mut position = q
                    .get_mut(entity)
                    .expect("requested entity for movement not found");

                position.translation = new_position;
            }
            NPCActionType::Attack {
                target,
                attacker_name,
            } => {
                dbg!("NPC attacking target!");
                match targets.get_mut(target) {
                    Ok((_, mut hp)) => {
                        hp.current -= 1;
                        log_writer.send(LogEvent::npc_attacks_player(attacker_name, 1));
                        info!(msg = "attacked player", ?hp)
                    }
                    Err(_) => error!("trying to attack non-existing entity"),
                };
            }
        }
    }
    game_state.set(GameState::PlayerTurn);
}
