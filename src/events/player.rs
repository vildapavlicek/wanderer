use bevy::prelude::*;

pub enum PlayerActionEvent {
    NoAction,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Attack(Entity),
}
