use crate::components::{Position, Size};
use bevy::prelude::*;

pub const ARENA_WIDTH: u32 = 11;
pub const ARENA_HEIGHT: u32 = 11;

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        let x = convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32);
        let y = convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32);
        transform.translation = Vec3::new(x, y, transform.translation.z);
    }
}
