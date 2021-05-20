#![windows_subsystem = "windows"] // disables console window
mod components;
mod events;
mod resources;
mod systems;

use crate::resources::GameState;
use crate::systems::{obstacle, player, ranged, PlayerSystems};
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
        .add_plugin(ranged::RangedPlugin)
        .add_startup_system(systems::setup.system())
        .add_startup_stage(
            "generate_map",
            SystemStage::single(systems::grid::generate_map.system()),
        )
        // .add_startup_system(
        //     systems::grid::generate_map
        //         .system()
        //         .label(SetupOrder::Map)
        //         .after(SetupOrder::Setup),
        // )
        // .add_startup_stage(
        //     "spawn_obstacle",
        //     SystemStage::single(obstacle::spawn_obstacles.system()),
        // )
        .add_system_set(
            SystemSet::on_update(GameState::EnemyTurn).with_system(
                systems::enemy::enemy_turn
                    .system()
                    .chain(systems::enemy::enemy_move.system()),
            ), // .after(PlayerSystems::PlayerMovement),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(systems::grid::position_translation.system()), // .with_system(systems::grid::size_scaling.system()),
        )
        .run();
}
