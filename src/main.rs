mod components;
mod events;
mod resources;
mod systems;

use crate::systems::{obstacle, player, PlayerSystems};
use bevy::prelude::*;

fn main() {
    // println!("Hello, world!");
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Lonely Wanderer".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.3)))
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugins)
        .add_startup_system(systems::setup.system())
        .add_startup_stage(
            "spawn_obstacle",
            SystemStage::single(obstacle::spawn_obstacles.system()),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(systems::grid::position_translation.system())
                .with_system(systems::grid::size_scaling.system()),
        )
        .run();
}
