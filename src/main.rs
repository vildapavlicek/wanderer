// #![windows_subsystem = "windows"] // disables console window
#![deny(clippy::float_cmp)]
#![allow(unused)]
mod components;
mod events;
mod map;
mod resources;
mod systems;

use crate::resources::GameState;
use crate::systems::{player, ranged};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    trace!("app started");
    println!("hello");

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Lonely Wanderer".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.3)))
        .init_resource::<systems::ui::LogMessages>()
        .add_event::<systems::ui::LogEvent>()
        .add_state(GameState::PlayerTurn)
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugins)
        .add_plugin(ranged::RangedPlugin)
        .add_plugin(EguiPlugin)
        .add_startup_system(systems::setup.system())
        .add_startup_stage(
            "generate_map",
            SystemStage::single(map::generate_map.system()), // systems::grid::generate_map.system()
        )
        .add_system_set(
            SystemSet::on_update(GameState::EnemyTurn).with_system(
                systems::enemy::enemy_turn
                    .system()
                    .chain(systems::enemy::enemy_move.system()),
            ),
        )
        .add_system(systems::animation.system())
        .add_system(systems::ui::update_logs.system())
        .add_system(systems::ui::ui.system())
        .add_system(systems::clear_dead.system())
        .run();
}
