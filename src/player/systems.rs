use crate::{
    DrillPickup, DrillSensor,
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
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

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
    gamepads: Query<(Entity, &Gamepad)>,
) {
    for (mut animation, mut sprite, speed, jump_force, mut velocity, mut state) in &mut query {
        let mut x = 0.0;
        let mut jump = false;
        for (_entity, gamepad) in &gamepads {
            x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
            jump = gamepad.just_pressed(GamepadButton::South);
        }
        // Horizontal movement
        let mut direction_x = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) || x < -0.1 {
            direction_x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || x > 0.1 {
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
            if keyboard_input.just_pressed(KeyCode::Space) || jump {
                velocity.linvel.y = jump_force.0;
                *state = PlayerState::Jumping;
                animation.animation = Animation::tag("jump")
                    .with_speed(1.0)
                    .with_repeat(AnimationRepeat::Count(1));
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

#[derive(Component, Default)]
pub struct DrillTool;

#[derive(Bundle, Default)]
pub struct DrillToolBundle {
    drill: DrillTool,
    transform: Transform,
    sprite: Sprite,
    aseprite: AseAnimation,
}

pub fn pickup_drill(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    drill_query: Query<(Entity, &ChildOf), With<DrillSensor>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, flags) => {
                if !flags.contains(CollisionEventFlags::SENSOR) {
                    continue;
                }
                let (player, drill) =
                    if player_query.get(*e1).is_ok() && drill_query.get(*e2).is_ok() {
                        (*e1, *e2)
                    } else if player_query.get(*e2).is_ok() && drill_query.get(*e1).is_ok() {
                        (*e2, *e1)
                    } else {
                        continue;
                    };
                if let Ok((_, child_of)) = drill_query.get(drill) {
                    // Despawn the parent entity
                    commands.entity(child_of.parent()).despawn();
                }
                println!("drill picked up");
            }
            CollisionEvent::Stopped(e1, e2, flags) => {
                if !flags.contains(CollisionEventFlags::SENSOR) {
                    continue;
                }
                let (player, drill) =
                    if player_query.get(*e1).is_ok() && drill_query.get(*e2).is_ok() {
                        (*e1, *e2)
                    } else if player_query.get(*e2).is_ok() && drill_query.get(*e1).is_ok() {
                        (*e2, *e1)
                    } else {
                        continue;
                    };
                println!("drill dropped");
            }
        }
        /*if let CollisionEvent::Started(e1, e2, flags) = event {
            if flags.contains(CollisionEventFlags::SENSOR) {
                let (player, drill) =
                    if player_query.get(*e1).is_ok() && drill_query.get(*e2).is_ok() {
                        (*e1, *e2)
                    } else if player_query.get(*e2).is_ok() && drill_query.get(*e1).is_ok() {
                        (*e2, *e1)
                    } else {
                        continue;
                    };

                // Despawn the pickup
                commands.entity(drill).despawn();

                let texture = asset_server.load("Drill.aseprite");

                // Spawn the tool drill in player's hands
                commands.entity(player).with_children(|parent| {
                    parent.spawn(DrillToolBundle {
                        drill: DrillTool,
                        transform: Transform::from_xyz(-0.1, 2.0, 0.0),
                        sprite: Sprite::default(),
                        aseprite: AseAnimation {
                            aseprite: texture,
                            animation: Animation::default().with_speed(0.0),
                        },
                    });
                });
            }
        }*/
    }
}

/*pub fn handle_ground_collision_events(
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
}*/

pub fn handle_ground_contacts(
    mut contact_events: EventReader<ContactForceEvent>,
    mut player_query: Query<&mut PlayerState, With<Player>>,
) {
    for event in contact_events.read() {
        for (other_entity, player_entity) in [
            (event.collider1, event.collider2),
            (event.collider2, event.collider1),
        ] {
            if let Ok(mut state) = player_query.get_mut(player_entity) {
                // Check if collision came from below
                let dir = event.max_force_direction.abs().normalize_or_zero();
                let from_below = dir.dot(Vec2::Y) > 0.6; // adjust threshold if needed

                if from_below {
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
            active_events: ActiveEvents::CONTACT_FORCE_EVENTS,
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_child(CameraBundle {
            camera2d: Camera2d,
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.3)),
            ..Default::default()
        });
}
