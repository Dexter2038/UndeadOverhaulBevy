use crate::{
    StaticBody,
    player::{
        bundle::{CameraBundle, PlayerBundle},
        components::{
            Direction, Inventory, InventoryUI, ItemsAround, JumpForce, Player, PlayerState,
            PushForce, Speed,
        },
    },
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

fn sprite_flip_x(sprite: &mut Sprite, val: bool) {
    if val == sprite.flip_x {
        return;
    }
    sprite.flip_x = val;
    if sprite.flip_x {
        sprite.anchor = bevy::sprite::Anchor::Custom(Vec2::new(0.25, -0.2));
    } else {
        sprite.anchor = bevy::sprite::Anchor::Custom(Vec2::new(-0.25, -0.2));
    }
}

pub fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut AseAnimation,
            &mut Sprite,
            &Speed,
            &JumpForce,
            &mut Velocity,
            &mut PlayerState,
        ),
        With<Player>,
    >,
) {
    for (mut animation, mut sprite, speed, jump_force, mut velocity, mut state) in &mut query {
        // Horizontal movement
        let mut direction_x = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction_x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction_x += 1.0;
        }
        velocity.linvel.x = direction_x * speed.0;

        match direction_x {
            -1.0 => {
                sprite_flip_x(sprite.as_mut(), true);
            }
            1.0 => {
                sprite_flip_x(sprite.as_mut(), false);
            }
            _ => {}
        }

        if state.as_ref() == &PlayerState::Idle || state.as_ref() == &PlayerState::Running {
            if keyboard_input.just_pressed(KeyCode::Space) {
                velocity.linvel.y = jump_force.0;
                *state = PlayerState::Jumping;
            } else {
                match direction_x {
                    -1.0 | 1.0 => {
                        animation.animation = Animation::tag("move")
                            .with_speed(1.0)
                            .with_repeat(AnimationRepeat::Loop);
                        *state = PlayerState::Running;
                    }
                    _ => {
                        animation.animation = Animation::tag("idel")
                            .with_speed(1.0)
                            .with_repeat(AnimationRepeat::Loop);
                        *state = PlayerState::Idle;
                    }
                }
            }
        }
    }
}

pub fn handle_ground_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_state: Query<&mut PlayerState, With<Player>>,
    ground_query: Query<(), With<StaticBody>>,
) {
    for event in collision_events.read() {
        let (e1, e2, started) = match event {
            CollisionEvent::Started(a, b, _) => (*a, *b, true),
            CollisionEvent::Stopped(a, b, _) => (*a, *b, false),
        };

        // Check player-ground interaction
        for &(player_entity, other_entity, is_start) in &[(e1, e2, started), (e2, e1, started)] {
            if let Ok(mut state) = player_state.get_mut(player_entity) {
                if ground_query.get(other_entity).is_ok() && is_start {
                    *state = PlayerState::Idle;
                }
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let inventory = Inventory {
        items: Vec::with_capacity(10),
        max_slots: 10,
    };
    let inventory_ui = InventoryUI(Entity::from_raw(0));
    let texture = asset_server.load("moveEngineer.aseprite");
    commands
        .spawn(PlayerBundle {
            player: Player,
            aseprite: AseAnimation {
                aseprite: texture,
                animation: Animation::tag("idel")
                    .with_speed(1.0)
                    .with_repeat(AnimationRepeat::Loop),
            },
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::Custom(Vec2::new(-0.25, -0.2)),
                ..Default::default()
            },
            speed: Speed(100.0),
            jump_force: JumpForce(200.0),
            push_force: PushForce(100.0),
            inventory,
            items_around: ItemsAround::default(),
            inventory_ui,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            collider: Collider::cuboid(6.0, 7.0),
            velocity: Velocity::zero(),
            direction: Direction::default(),
            state: PlayerState::default(),
            gravity: GravityScale::default(),
            friction: Friction::new(0.0),
            rigidbody: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_events: ActiveEvents::COLLISION_EVENTS,
        })
        .with_child(CameraBundle {
            camera2d: Camera2d,
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.3)),
            ..Default::default()
        });
}
