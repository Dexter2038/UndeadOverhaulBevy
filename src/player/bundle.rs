use crate::player::components::*;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub speed: Speed,
    pub jump_force: JumpForce,
    pub push_force: PushForce,
    pub inventory: Inventory,
    pub items_around: ItemsAround,
    pub inventory_ui: InventoryUI,
    pub sprite: Sprite,
    pub aseprite: AseAnimation,
    pub transform: Transform,
    pub velocity: Velocity,
    pub state: PlayerState,
    pub direction: Direction,
    pub collider: Collider,
    pub gravity: GravityScale,
    pub friction: Friction,
    pub rigidbody: RigidBody,
    pub locked_axes: LockedAxes,
    pub active_events: ActiveEvents,
}

#[derive(Bundle, Default)]
pub struct CameraBundle {
    pub camera2d: Camera2d,
    pub camera: Camera,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
