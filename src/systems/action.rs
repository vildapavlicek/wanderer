use crate::components::{npc::*, player::*, *};
use bevy::prelude::*;

pub struct MovementEvent {
    actor: Entity,
    destination: Vec3,
}

pub fn resolve_attack_or_move(
    mut event_reader: EventReader<MovementEvent>,
    blockers: Query<(Entity, &Transform, Option<&Health>), With<Blocks>>,
) {
    if let Some(movement_event) = event_reader.iter().next() {}
}
