use crate::components::player::Player;
use crate::components::Enemy;
use crate::resources::GameState;
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::{Commands, Query, With};
use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct PlayerDistance;

impl PlayerDistance {
    pub fn build() -> PlayerDistanceBuilder {
        PlayerDistanceBuilder
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PlayerDistanceBuilder;

impl ScorerBuilder for PlayerDistanceBuilder {
    fn build(&self, cmd: &mut Commands, scorer: Entity, actor: Entity) {
        cmd.entity(scorer).insert(PlayerDistance);
    }
}

pub fn should_move_scorer(
    player: Query<&Transform, With<Player>>,
    movers: Query<(Entity, &Transform), With<Enemy>>,
    mut query: Query<(&Actor, &mut Score), With<PlayerDistance>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let player_translation = player.single().expect("no player found").translation;

    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok((entity, npc_transform)) = movers.get(*actor) {
            let npc_translation = npc_transform.translation;
            let pos_diff = player_translation - npc_translation;

            if player_translation.abs_diff_eq(npc_translation, 32.) {
                score.set(1.);
            } else {
                let range = npc_translation - player_translation;
                let (x, y) = (range.x / 32., range.y / 32.);
                let score_to_be = 1. - ((x.abs() + y.abs()) / 100.);
                score.set(score_to_be);
            }
        }
    }
}