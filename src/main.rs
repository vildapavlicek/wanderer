#![windows_subsystem = "windows"] // disables console window, disable in VSCode, otherwise there is no output in console
#![allow(clippy::float_cmp)]
#![allow(clippy::type_complexity)]
#![allow(unused)]
mod ai;
mod components;
mod map;
mod resources;
mod systems;

use crate::resources::GameState;
use crate::systems::{player, ranged, PlayerSystems};
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use big_brain::BigBrainPlugin;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Lonely Wanderer".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .init_resource::<systems::ui::LogMessages>()
        .add_event::<systems::ui::LogEvent>()
        .add_state(GameState::PlayerTurn)
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugins)
        .add_plugin(ranged::RangedPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(BigBrainPlugin)
        .add_startup_system(systems::setup.system())
        .add_startup_system(update_logging.system())
        .add_startup_stage(
            "generate_map",
            SystemStage::single(map::generate_map.system()), // systems::grid::generate_map.system()
        )
        // .add_system_set(
        //     SystemSet::on_update(GameState::EnemyScorer)
        //         .with_system(ai::scorers::player_distance_scorer_system.system()),
        // )
        .add_system(ai::scorers::should_move_scorer.system())
        // .add_system(ai::scorers::should_skip_scorer.system())
        .add_system_set(
            SystemSet::on_update(GameState::EnemyTurn)
                .with_system(
                    systems::enemy::enemy_movement
                        .system()
                        .chain(systems::enemy::enemy_move.system()),
                )
                .with_system(systems::enemy::enemy_skip_turn.system())
                .with_system(systems::enemy::resolve_end_turn.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PrepareEnemyTurn)
                .with_system(systems::enemy::prepare_enemy_turn.system()),
        )
        .add_system(systems::animation.system())
        .add_system(systems::ui::update_logs.system())
        .add_system(systems::ui::ui.system())
        .add_system(systems::clear_dead.system())
        .add_system(systems::cheats.system())
        .run();
}

fn update_logging(mut settings: ResMut<LogSettings>) {
    settings.filter = String::from("wanderer=trace,wgpu=error");
    settings.level = Level::TRACE;

    info!("hello world");
}
