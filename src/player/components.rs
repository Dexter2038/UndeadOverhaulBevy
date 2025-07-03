#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Speed(pub f32);

#[derive(Component, Default)]
pub struct JumpForce(pub f32);

#[derive(Component, Default)]
pub struct PushForce(pub f32);

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub id: String,
    pub quantity: u32,
}

#[derive(Component, Default)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    pub max_slots: u32,
}

#[derive(Component, Default)]
pub struct ItemsAround(Vec<Entity>);

#[derive(Component)]
pub struct InventoryUI(pub Entity);

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum PlayerState {
    #[default]
    Idle,
    Running,
    Jumping,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum Direction {
    Left,
    #[default]
    Right,
}
