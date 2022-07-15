use bevy::{
    asset::Asset,
    ecs::system::Command,
    prelude::{Component, Plugin, ResMut},
};

/// This is marker component that helps us query data related to player
#[derive(Debug, Component)]
pub struct Player;

/// This is helper marker component that helps us query data realted to player camera
#[derive(Debug, Component)]
pub struct PlayerCamera;
