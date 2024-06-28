use crate::ai::actions::Move;
use crate::components::{player::Player, Blocking, Enemy, Health};
use crate::resources::GameState;
use bevy::prelude::*;
use big_brain::actions::ActionState;
use big_brain::prelude::Actor;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyTurnSet;

#[derive(Debug)]
pub enum NPCActionType {
    Move {
        actor: Entity,
        x: f32,
        y: f32,
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
    mut actors: Query<(&Actor, &mut ActionState), With<Move>>,
    blockers: Query<(&Transform, &Blocking), Without<Player>>,
) -> Vec<NPCActionType> {
    let mut to_move: Vec<NPCActionType> = vec![];

    let (player_entity, player_pos) = player.single();

    for (Actor(actor), mut action_state) in actors.iter_mut() {
        if let Ok((entity, npc_transform, name)) = enemies.get(*actor) {
            match *action_state {
                big_brain::actions::ActionState::Requested => {
                    let mut possible_blockers = blockers
                        .iter()
                        .filter_map(|(b_trans, _)| {
                            if npc_transform.translation.x + super::SPRITE_SIZE
                                == b_trans.translation.x
                                || npc_transform.translation.x - super::SPRITE_SIZE
                                    == b_trans.translation.x
                                || npc_transform.translation.y + super::SPRITE_SIZE
                                    == b_trans.translation.y
                                || npc_transform.translation.y - super::SPRITE_SIZE
                                    == b_trans.translation.y
                            {
                                Some(b_trans.to_owned())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<Transform>>();

                    possible_blockers.append(
                        &mut to_move
                            .iter()
                            .filter_map(|action| {
                                if let NPCActionType::Move {
                                    actor: entity,
                                    x,
                                    y,
                                } = action
                                {
                                    Some(Transform::from_xyz(*x, *y, super::MONSTER_LAYER))
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<Transform>>(),
                    );

                    let Some(future_pos) =
                        resolve_position(npc_transform, player_pos, possible_blockers)
                    else {
                        *action_state = big_brain::actions::ActionState::Success;
                        continue;
                    };

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
                        x: future_pos.x,
                        y: future_pos.y,
                    });

                    *action_state = big_brain::actions::ActionState::Success;
                }
                _ => debug!(action_state = format!("{:?}", *action_state).as_str()),
            }
        }
    }

    to_move
}

fn resolve_position(npc: &Transform, player: &Transform, blockers: Vec<Transform>) -> Option<Vec3> {
    // if player is right to the npc
    if player.translation.x > npc.translation.x
        && !blockers.iter().any(|pos| {
            pos.translation.x == npc.translation.x + super::SPRITE_SIZE
                && pos.translation.y == npc.translation.y
        })
    {
        return Some(Vec3::new(
            npc.translation.x + super::SPRITE_SIZE,
            npc.translation.y,
            npc.translation.z,
        ));
    }

    // if player is left to the npc
    if player.translation.x < npc.translation.x
        && !blockers.iter().any(|pos| {
            pos.translation.x == npc.translation.x - super::SPRITE_SIZE
                && pos.translation.y == npc.translation.y
        })
    {
        return Some(Vec3::new(
            npc.translation.x - super::SPRITE_SIZE,
            npc.translation.y,
            npc.translation.z,
        ));
    }

    // if player is above the npc
    if player.translation.y > npc.translation.y
        && !blockers.iter().any(|pos| {
            pos.translation.y == npc.translation.y + super::SPRITE_SIZE
                && pos.translation.x == npc.translation.x
        })
    {
        return Some(Vec3::new(
            npc.translation.x,
            npc.translation.y + super::SPRITE_SIZE,
            npc.translation.z,
        ));
    }

    // if player is bellow the npc
    if player.translation.y < npc.translation.y
        && !blockers.iter().any(|pos| {
            pos.translation.y == npc.translation.y - super::SPRITE_SIZE
                && pos.translation.x == npc.translation.x
        })
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
                x,
                y,
            } => {
                trace!(?entity, ?x, ?y, "moving NPC");
                let mut position = q
                    .get_mut(entity)
                    .expect("requested entity for movement not found");

                position.translation = Vec3::new(x, y, position.translation.z);
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
