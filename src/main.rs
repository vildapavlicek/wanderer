mod components;
mod events;
mod resources;
mod systems;

use crate::events::player::PlayerActionEvent;
use crate::systems::{obstacle, player, PlayerSystems};
use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.3)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup.system())
        .add_startup_stage(
            "spawn_player",
            SystemStage::single(player::spawn_player.system()),
        )
        .add_startup_stage(
            "spawn_obstacle",
            SystemStage::single(obstacle::spawn_obstacle.system()),
        )
        // .add_system(systems::player::handle_key_input.system())
        .add_system_set(
            SystemSet::new()
                .with_system(
                    player::handle_key_input
                        .system()
                        .label(PlayerSystems::HandleInput),
                )
                .with_system(
                    player::player_movement
                        .system()
                        .label(PlayerSystems::PlayerMovement)
                        .after(PlayerSystems::HandleInput),
                ),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(systems::grid::position_translation.system())
                .with_system(systems::grid::size_scaling.system()),
        )
        .add_event::<PlayerActionEvent>()
        .run();
}
