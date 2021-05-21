#![windows_subsystem = "windows"] // disables console window
mod components;
mod events;
mod resources;
mod systems;

use crate::resources::GameState;
use crate::systems::{player, ranged, PlayerSystems};
use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum SetupOrder {
    Setup,
    Map,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Lonely Wanderer".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.3)))
        .add_state(GameState::PlayerTurn)
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugins)
        // .add_plugin(ranged::RangedPlugin)
        .add_startup_system(systems::setup.system())
        .add_startup_stage(
            "generate_map",
            SystemStage::single(systems::grid::generate_map.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::EnemyTurn).with_system(
                systems::enemy::enemy_turn
                    .system()
                    .chain(systems::enemy::enemy_move.system()),
            ), // .after(PlayerSystems::PlayerMovement),
        )
        .add_system(systems::animation.system())
        .run();
}
