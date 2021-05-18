use crate::components::{Enemy, Health, Position};
use crate::resources::GameState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum RangedAttackOrder {
    Target,
    Attack,
}

pub struct RangedPlugin;

impl Plugin for RangedPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::RangedTargeting)
                .with_system(targeting.system().label(RangedAttackOrder::Target))
                .with_system(
                    ranged_attack
                        .system()
                        .label(RangedAttackOrder::Attack)
                        .after(RangedAttackOrder::Target),
                ),
        )
        .add_event::<RangedAttackEvent>();
    }
}

pub struct RangedAttackEvent {
    x: i32,
    y: i32,
}

pub struct TargetArea {
    x: i32,
    y: i32,
}

impl std::default::Default for TargetArea {
    fn default() -> TargetArea {
        TargetArea { x: 0, y: 0 }
    }
}

pub fn targeting(
    mut game_state: ResMut<State<GameState>>,
    mut target: Local<TargetArea>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut mouse_input: ResMut<Input<MouseButton>>,
    mut mouse_moved_event: EventReader<CursorMoved>,
    mut attack_event_writer: EventWriter<RangedAttackEvent>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        key_input.update();
        game_state.set(GameState::PlayerTurn);
        return;
    }

    if let Some(moved_event) = mouse_moved_event.iter().next() {
        info!(?moved_event);
        let position: Vec2 = moved_event.position;
        let x_coord = (position.x / 50.).round();
        let y_coord = (position.y / 50.).round();

        *target = TargetArea {
            x: x_coord as i32,
            y: y_coord as i32,
        };
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        attack_event_writer.send(RangedAttackEvent {
            x: target.x,
            y: target.y,
        });
    }
}

fn ranged_attack(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut attack_event_reader: EventReader<RangedAttackEvent>,
    mut query: Query<(Entity, &Position, &mut Health), With<Enemy>>,
) {
    if let Some(attack_target) = attack_event_reader.iter().next() {
        if let Some((entity, pos, mut health)) = query
            .iter_mut()
            .find(|(_, pos, _)| pos.x == attack_target.x && pos.y == attack_target.y)
        {
            health.current -= 1;
            info!(
                msg = "attacking at enemy",
                pos.x,
                pos.y,
                hp = health.current
            );

            if health.current <= 0 {
                commands.entity(entity).despawn();
            }

            game_state.set(GameState::EnemyTurn).unwrap();
        }
    }
}
