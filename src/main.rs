use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, Material2dProperties},
};
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

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
pub struct GlowMaterial {
    #[uniform(0)]
    pub hovered: f32,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for GlowMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glow_material.wgsl".into()
    }
}

#[derive(Component)]
pub struct DrillPickup;

#[derive(Bundle)]
struct DrillBundle {
    drill: DrillPickup,
    rigidbody: RigidBody,
    collider: Collider,
    transform: Transform,
    sprite: Sprite,
    aseprite: AseAnimation,
    material_sprite: MeshMaterial2d<GlowMaterial>,
}

#[derive(Component)]
pub struct DrillSensor;

#[derive(Bundle)]
struct DrillSensorBundle {
    drill: DrillSensor,
    collider: Collider,
    transform: Transform,
    active_events: ActiveEvents,
    sensor: Sensor,
}

fn create_drill(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<GlowMaterial>>,
) {
    let texture = asset_server.load("Drill.aseprite");
    commands
        .spawn(DrillBundle {
            drill: DrillPickup,
            rigidbody: RigidBody::Dynamic,
            collider: Collider::cuboid(10.0, 4.0),
            transform: Transform::from_xyz(30.0, 10.0, 0.0),
            sprite: Sprite::default(),
            aseprite: AseAnimation {
                aseprite: texture,
                animation: Animation::default().with_speed(0.0),
            },
            material_sprite: MeshMaterial2d(materials.add(GlowMaterial {
                hovered: 1.0,
                ..Default::default() //texture: texture.clone(),
            })),
        })
        .with_children(|parent| {
            parent.spawn(DrillSensorBundle {
                drill: DrillSensor,
                collider: Collider::cuboid(14.0, 7.0),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                active_events: ActiveEvents::COLLISION_EVENTS,
                sensor: Sensor,
            });
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
        .add_plugins(Material2dPlugin::<GlowMaterial>::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, create_static_body)
        .add_systems(Startup, create_drill)
        .run();
}
