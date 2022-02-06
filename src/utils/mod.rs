use bevy::prelude::Transform;

fn distance(pos1: &Transform, pos2: &Transform) -> f32 {
    let dx = f32::abs(pos2.translation.x - pos1.translation.x);
    let dy = f32::abs(pos2.translation.y - pos1.translation.y);

    let min = f32::min(dx, dy);
    let max = f32::max(dx, dy);

    let diagonal_steps = min;
    let straight_steps = max - min;

    /* std::f32::consts::SQRT_2 * */
    diagonal_steps + straight_steps
}
