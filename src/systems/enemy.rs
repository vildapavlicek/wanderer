use crate::ai::actions::{Move, Skip};
use crate::components::{player::Player, Blocking, Enemy, Health};
use crate::resources::GameState;
use bevy::prelude::*;
use big_brain::actions::ActionState;
use big_brain::prelude::Actor;

pub enum MoveDirection {
    Left,
    Right,
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
}

pub struct CanAct;

use crate::ai::scorers::PlayerDistance;
use crate::components::ItemName;

pub fn enemy_movement(
    mut cmd: Commands,
    player: Query<(Entity, &Transform), With<Player>>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, With<CanAct>)>,
    mut actors: Query<(&Actor, &mut ActionState), With<Move>>,
    blockers: Query<(&Transform, &Blocking)>,
) -> Option<(Entity, Vec3)> {
    trace!("enemy movement system");
    let (player_entity, player_pos) = player.single().expect("no player entity");

    let movement_actors = actors.iter_mut().count();
    debug!(?movement_actors, "still have movement actors");

    for (Actor(actor), mut state) in actors.iter_mut() {
        cmd.entity(*actor).remove::<CanAct>();
        match *state {
            ActionState::Requested => {
                if let Ok((entity, npc_transform)) = enemies.get(*actor) {
                    trace!(?entity, ?npc_transform, "moving enemy");
                    let possible_blockers = blockers
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

                    return match resolve_position(npc_transform, player_pos, possible_blockers) {
                        Some(fut_pos) => {
                            debug!(?entity, ?actor, ?npc_transform, "executing movement");
                            *state = ActionState::Executing;
                            Some((entity, fut_pos))
                        }
                        None => {
                            debug!(?entity, ?actor, ?npc_transform, "no movement");
                            *state = ActionState::Success;
                            None
                        }
                    };
                } else {
                    debug!(?actor, "got actor but not enemy");
                    *state = ActionState::Success;
                }
            }
            _ => *state = ActionState::Success,
        }
    }
    None
}

use crate::systems::ui::LogEvent;

pub fn enemy_move(
    In(mover): In<Option<(Entity, Vec3)>>,

    mut q: Query<(Entity, &mut Transform)>,
    mut actors: Query<(&Actor, &mut ActionState), With<Move>>,
) {
    if let Some(mover) = mover {
        if let Ok((_, mut transform)) = q.get_mut(mover.0) {
            transform.translation = mover.1;

            if let Some((Actor(actor), mut state)) = actors
                .iter_mut()
                .find(|(Actor(actor), _)| &mover.0 == actor)
            {
                debug!(?actor, ?state, "setting state to success");
                *state = ActionState::Success;
            }
        }
    }
}

pub fn enemy_skip_turn(
    mut cmd: Commands,
    mut can_act: Query<(Entity, &Transform), (With<CanAct>, With<Enemy>)>,
    mut actors: Query<(&Actor, &mut ActionState), With<Skip>>,
) {
    for (Actor(actor), mut state) in actors.iter_mut() {
        cmd.entity(*actor).remove::<CanAct>();
        if let Ok((e, transform)) = can_act.get(*actor) {
            trace!(?e, ?transform, "skipping turn");
            *state = ActionState::Success;
        }
    }
}

pub fn resolve_end_turn(
    query: Query<Entity, (With<CanAct>, With<Enemy>)>,
    mut game_state: ResMut<State<GameState>>,
) {
    let count = query.iter().count();
    debug!(?count, "still can act");
    if count == 0 {
        game_state
            .set(GameState::PlayerTurn)
            .expect("failed to set GameState to PlayerTurn");
    }
}

pub fn prepare_enemy_turn(
    mut cmd: Commands,
    mut game_state: ResMut<State<GameState>>,
    query: Query<Entity, (With<Enemy>, Without<CanAct>)>,
) {
    for e in query.iter() {
        cmd.entity(e).insert(CanAct);
    }

    game_state
        .set(GameState::EnemyTurn)
        .expect("failed to set GameState to PlayerTurn");
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