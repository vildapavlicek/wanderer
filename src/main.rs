// #![windows_subsystem = "windows"] // disables console window, disable in VSCode, otherwise there is no output in console
#![allow(clippy::float_cmp)]
#![allow(clippy::type_complexity)]
mod ai;
mod components;
mod map;
mod resources;
mod systems;

use crate::resources::GameState;
use crate::systems::{player, ranged};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use big_brain::BigBrainPlugin;
use map::MapGenSet;
use systems::enemy::EnemyTurnSet;
use systems::{mark_dead, SetupSet};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .init_resource::<systems::ui::LogMessages>()
        .add_event::<systems::ui::LogEvent>()
        .insert_state(GameState::PlayerTurn)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Lonely Wanderer".to_string(),
                    resolution: (1024f32, 768f32).into(),
                    ..default()
                }),
                ..default()
            }),
            player::PlayerPlugins,
            ranged::RangedPlugin,
            EguiPlugin,
            BigBrainPlugin::new(PreUpdate),
        ))
        .add_systems(
            Startup,
            (
                systems::setup.in_set(SetupSet),
                map::generate_map.run_if(run_once()).in_set(MapGenSet),
            ),
        )
        .add_systems(
            Update,
            ai::scorers::player_in_range_scorer_system
                .run_if(in_state(GameState::EnemyTurn))
                .in_set(ai::scorers::NpcScorerSet),
        )
        .add_systems(
            Update,
            systems::enemy::enemy_turn
                .pipe(systems::enemy::enemy_move)
                .run_if(in_state(GameState::EnemyTurn))
                .in_set(EnemyTurnSet),
        )
        .add_systems(OnExit(GameState::PlayerTurn), mark_dead)
        .configure_sets(Startup, SetupSet.before(MapGenSet))
        .configure_sets(Update, EnemyTurnSet.after(ai::scorers::NpcScorerSet))
        .add_systems(
            Update,
            (
                systems::animation,
                systems::ui::update_logs,
                systems::ui::ui,
                systems::clear_dead,
                systems::cheats,
            ),
        )
        .run();
}
