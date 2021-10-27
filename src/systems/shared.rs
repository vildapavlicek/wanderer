pub(super) fn is_out_of_bounds(x: f32, y: f32, max_x: usize, max_y: usize) -> bool {
    x < 0. || x >= (max_x * 32) as f32 || y < 0. || y >= (max_y * 32) as f32
}
