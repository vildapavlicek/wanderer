use crate::components::{player::PlayerCamera, Enemy, Health};
use crate::resources::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct RangedPlugin;

impl Plugin for RangedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            targeting
                .pipe(ranged_attack)
                .run_if(in_state(GameState::RangedTargeting)),
        );
    }
}

#[derive(Default, Debug)]
pub struct RangedAttackEvent {
    x: f32,
    y: f32,
}

#[derive(Default, Debug)]
pub struct TargetLocation {
    x: f32,
    y: f32,
}

pub fn targeting(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut target: Local<TargetLocation>,
    mut key_input: ResMut<ButtonInput<KeyCode>>,
    mouse_input: ResMut<ButtonInput<MouseButton>>,
    q_camera: Query<&Transform, With<PlayerCamera>>,
) -> Option<RangedAttackEvent> {
    if key_input.just_pressed(KeyCode::Escape) {
        // key_input.update();
        game_state.set(GameState::PlayerTurn);
        return None;
    }

    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    let window = primary_window.single();

    if let Some(pos) = window.cursor_position() {
        // get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        *target = TargetLocation {
            x: pos_wld.x,
            y: pos_wld.y,
        };
    }

    match mouse_input.just_pressed(MouseButton::Left) {
        true => Some(RangedAttackEvent {
            x: target.x,
            y: target.y,
        }),
        false => None,
    }
}

// todo I think we should have single system handling all the attacks and not have one for melee and one for ranged
use super::ui::LogEvent;
use crate::components::ItemName;
fn ranged_attack(
    In(target): In<Option<RangedAttackEvent>>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut query: Query<(Entity, &Transform, &mut Health, &ItemName), With<Enemy>>,
    mut log_writer: EventWriter<LogEvent>,
) {
    if let Some(attack_target) = target {
        let (x, y) = get_coords(attack_target.x, attack_target.y);
        if let Some((entity, _, mut health, name)) =
            query.iter_mut().find(|(_, transform, _, _)| {
                transform.translation.x == x && transform.translation.y == y
            })
        {
            health.current -= 1;

            log_writer.send(LogEvent::player_attack(name.to_string(), 1));

            if health.current <= 0 {
                commands.entity(entity).despawn();
            }

            game_state.set(GameState::EnemyTurn);
        }
    }
}

use crate::map::SPRITE_SIZE;
fn get_coords(x: f32, y: f32) -> (f32, f32) {
    let x2 = (x / SPRITE_SIZE).round() * SPRITE_SIZE;
    let y2 = (y / SPRITE_SIZE).round() * SPRITE_SIZE;
    (x2, y2)
}
