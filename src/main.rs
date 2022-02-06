// #![windows_subsystem = "windows"] // disables console window, disable in VSCode, otherwise there is no output in console
#![allow(clippy::float_cmp)]
#![allow(clippy::type_complexity)]
#![allow(unused)]
mod ai;
mod components;
mod map;
mod resources;
mod systems;
mod utils;

use crate::resources::GameState;
use crate::systems::{player, ranged};
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use big_brain::BigBrainPlugin;

fn main() {
    App::new()
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
        .add_startup_system(systems::setup)
        .add_startup_system(update_logging)
        .add_startup_stage("generate_map", SystemStage::single(map::generate_map))
        .add_system_set(
            SystemSet::on_update(GameState::EnemyTurn)
                .with_system(ai::scorers::player_in_range_scorer_system)
                .label("npc_scorer"),
        )
        .add_system_set(
            SystemSet::on_update(GameState::EnemyTurn)
                .with_system(
                    systems::enemy::enemy_turn
                        .system()
                        .chain(systems::enemy::enemy_move),
                )
                .label(systems::SystemLabel::EnemyTurn)
                .after("npc_scorer"),
        )
        .add_system(systems::animation)
        .add_system(systems::ui::update_logs)
        .add_system(systems::ui::ui)
        .add_system(systems::clear_dead)
        .add_system(systems::cheats)
        .run();
}

fn update_logging(mut settings: ResMut<LogSettings>) {
    settings.filter = String::from("wanderer=trace,wgpu=error");
    settings.level = Level::TRACE;

    info!("hello world");
}
