use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;

use crate::player::PlayerPlugin;

mod player;

#[derive(Component)]
pub struct StaticBody;

#[derive(Bundle)]
struct StaticBodyBundle {
    static_body: StaticBody,
    rigidbody: RigidBody,
    collider: Collider,
    transform: Transform,
}

fn create_static_body(mut commands: Commands) {
    commands.spawn(StaticBodyBundle {
        static_body: StaticBody,
        rigidbody: RigidBody::Fixed,
        collider: Collider::cuboid(100.0, 10.0),
        transform: Transform::from_xyz(0.0, -100.0, 0.0),
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, create_static_body)
        .run();
}
