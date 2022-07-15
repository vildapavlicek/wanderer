// #![windows_subsystem = "windows"] // disables console window, disable in VSCode, otherwise there is no output in console
#![allow(clippy::float_cmp)]
#![allow(clippy::type_complexity)]
#![allow(unused)]
mod components;
mod map;
mod resources;
mod systems;

use crate::resources::GameState;
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use big_brain::BigBrainPlugin;
use systems::{player::PlayerPlugin, sandbox::SandboxPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Lonely Wanderer".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(SandboxPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        // .init_resource::<systems::ui::LogMessages>()
        // .add_event::<systems::ui::LogEvent>()
        // .add_state(GameState::PlayerTurn)
        //.add_plugin(ranged::RangedPlugin)
        //.add_plugin(EguiPlugin)
        //.add_plugin(BigBrainPlugin)
        .add_startup_system(systems::setup)
        .add_startup_system(update_logging)
        //.add_startup_stage("generate_map", SystemStage::single(map::generate_map))
        /* .add_system_set(
            SystemSet::on_update(GameState::EnemyTurn)
                .with_system(ai::scorers::player_in_range_scorer_system)
                .label("npc_scorer"),
        )
        .add_system_set(
            SystemSet::on_update(GameState::EnemyTurn)
                .with_system(systems::enemy::enemy_turn.chain(systems::enemy::enemy_move))
                .after("npc_scorer"),
        ) */
        //.add_system(systems::animation)
        /* .add_system(systems::ui::update_logs)
        .add_system(systems::ui::ui)
        .add_system(systems::clear_dead) */
        //.add_system(systems::cheats)
        .run();
}

fn update_logging(mut settings: ResMut<LogSettings>) {
    settings.filter = String::from("wanderer=trace,wgpu=error");
    settings.level = Level::TRACE;

    info!("hello world");
}
/*
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
} */
