use bevy::prelude::*;
use std::ops::Neg;

const STEP: f32 = 1.;

#[derive(Debug, Component)]
pub struct Movement {
    direction: MoveDirection,
    entity: Entity,
}

impl Movement {
    pub fn new(direction: MoveDirection, entity: Entity) -> Self {
        Movement { direction, entity }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MoveDirection {
    Left,
    LeftUp,
    Up,
    RightUp,
    Right,
    DownRight,
    Down,
    DownLeft,
}

impl MoveDirection {
    pub fn r#move(&self, transform: &mut Transform, distance: Option<f32>) {
        transform.translation.x += self.move_on_x(distance.unwrap_or(STEP)) * super::SPRITE_SIZE;
        transform.translation.y += self.move_on_y(distance.unwrap_or(STEP)) * super::SPRITE_SIZE;
    }

    /// Resolves whether to Add (move up) to X or Subtract (move down)
    fn move_on_y(&self, distance: f32) -> f32 {
        match self {
            Self::LeftUp | Self::Up | Self::RightUp => distance,
            Self::DownLeft | Self::Down | Self::DownRight => distance.neg(),
            _ => 0.,
        }
    }

    /// Resolves whether to Add (mover right) to Y or Subtract (move left)
    fn move_on_x(&self, distance: f32) -> f32 {
        match self {
            Self::Left | Self::LeftUp | Self::DownLeft => distance.neg(),
            Self::Right | Self::RightUp | Self::DownRight => distance,
            _ => 0.,
        }
    }

    pub fn get_new_translation(&self, transform: &Transform, distance: Option<f32>) -> Vec3 {
        let distance = distance.unwrap_or(STEP);
        let translation = transform.translation;
        Vec3::new(
            translation.x + (self.move_on_x(distance) * super::SPRITE_SIZE),
            translation.y + (self.move_on_y(distance) * super::SPRITE_SIZE),
            transform.translation.z,
        )
    }
}
