
use bevy::prelude::*;

use crate::{player::*, camera::MovementHelper, platform::Layer};
use bevy_xpbd_3d::{math::*, prelude::*};

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
                update_grounded,
                //keyboard_input,
                gamepad_input,
                handle_wrench_swing,
                remove_hitbox,
                handle_gliding
            ).chain()
        );
    }
}


#[derive(Component)]
pub struct Jump;

#[derive(Component)]
pub struct DoubleJump;

#[derive(Component)]
pub struct HighJump;

#[derive(Component)]
pub struct Falling;

#[derive(Component)]
pub struct Gliding;

#[derive(Component)]
pub struct Walking;

#[derive(Component)]
pub struct Crouch;

#[derive(Component)]
pub struct Idle;

#[derive(Component)]
pub struct Swing1 {
    pub swing_time: f32
}

#[derive(Component)]
pub struct Hitbox;

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct Direction(pub Vec2);

#[derive(Component)]
pub struct LongJump;

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
    camera: Query<&Transform, With<MovementHelper>>,
    mut player: Query<(
        Entity, 
        &mut Transform,
        &mut Direction, 
        Has<Swing1>, 
        Has<Grounded>,
        Has<Falling>,
        Has<Jump>,
        Has<DoubleJump>,
        Has<Crouch>,
        Has<HighJump>,
        Has<LongJump>,
        &JumpCounter
    ), (With<CharacterController>, Without<MovementHelper>)>,
    mouse_input: Res<Input<MouseButton>>,

    mut commands: Commands,
    time: Res<Time>
) {
    let Ok(camera_transform) = camera.get_single() else {return;};
    let Ok((player_entity, 
        mut player_transform,
        mut direction, 
        is_swinging, 
        is_grounded,
        is_falling,
        is_jumping,
        is_double_jumping,
        is_crouching,
        is_high_jumping,
        is_long_jumping,
        jump_counter
    )) = player.get_single_mut() else {return;};

    // calculates direction
    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

    let vertical = (up as i8 - down as i8) as f32 * Vec2::new(camera_transform.forward().x, -camera_transform.forward().z);
    let horizontal =  (right as i8 - left as i8) as f32 * Vec2::new(camera_transform.right().x, -camera_transform.right().z);
   
    if !is_swinging {
        direction.0 = horizontal + vertical;
    }

    // if the player is idle it doesn't move,
    // else it looks in the direction and 
    // if it's not crouching it moves
    if direction.0 == Vec2::ZERO {
        commands.entity(player_entity).insert(Idle);
        commands.entity(player_entity).remove::<Walking>();

        if is_swinging {
            let swing_direction = Vec2::new(
                -player_transform.forward().x,
                player_transform.forward().z
            ).normalize();
            
            movement_event_writer.send(MovementAction::Swing1(swing_direction / 4.));
        }
    }
    else {
        
        let look_at = player_transform.translation + Vec3::new(-direction.0.x, 0., direction.0.y);
        player_transform.look_at(look_at, Vec3::Y);
        
        if !is_crouching {
            commands.entity(player_entity).insert(Walking);
            commands.entity(player_entity).remove::<Idle>();
            movement_event_writer.send(MovementAction::Move(direction.0));
        }
    }
    
    // wrench swing
    if mouse_input.any_just_pressed([MouseButton::Middle]) && !is_swinging {
            
        // inserts the swing state
        commands.entity(player_entity).insert(Swing1{
            swing_time: time.elapsed_seconds()
        });

        // [HITBOX]

        // creates a collider that interacts only with hittable entities
        let hitbox = commands.spawn((
            Collider::cylinder(0.5, 1.2),
            CollisionLayers::new([Layer::Player], [Layer::Hittable]),
            Hitbox,
            Damage(1.)
        )).id(); 

        // positions the collider in front of the player
        let hitbox_position = Position::new(
            player_transform.translation + Vec3::new(
                -player_transform.forward().x , 
                player_transform.forward().y, 
                player_transform.forward().z
            ).normalize() / 2.
        );

        // spawns the collider
        commands.entity(hitbox).insert(hitbox_position);
    }

    

    // crouch
    if keyboard_input.pressed(KeyCode::ShiftLeft) && is_grounded {
        commands.entity(player_entity).insert(Crouch);
    }
    else {
        commands.entity(player_entity).remove::<Crouch>();
    }

    // Jumps and HighJump
    if !is_crouching {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if jump_counter.counter == 0. && is_grounded {
                commands.entity(player_entity).insert(Jump);
                movement_event_writer.send(MovementAction::Jump);
            }
            else {
                if jump_counter.counter < 2. && jump_counter.counter > 0.
                && time.elapsed_seconds() < jump_counter.jump_time + 0.67  {
                    movement_event_writer.send(MovementAction::DoubleJump);
                    commands.entity(player_entity).remove::<Jump>();
                    commands.entity(player_entity).insert(DoubleJump);
                    
                }
                
            }
        }
    } else {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if direction.0 == Vec2::ZERO {
                commands.entity(player_entity).insert(HighJump);
                movement_event_writer.send(MovementAction::HighJump);
            }
            else {
                commands.entity(player_entity).insert(LongJump);
                movement_event_writer.send(MovementAction::LongJump(direction.0));
            }
        }

    }

    // gliding
    if is_falling && !is_jumping && !is_double_jumping && !is_high_jumping {
        if keyboard_input.pressed(KeyCode::Space) {
            commands.entity(player_entity).insert(Gliding);
        }
        else {
            commands.entity(player_entity).remove::<Gliding>();
        }
    }

}




/// Sends [`MovementAction`] events based on gamepad input.
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    camera: Query<&Transform, With<MovementHelper>>,
    mut player: Query<(
        Entity, 
        &mut Transform, 
        &mut Direction,
        Has<Swing1>, 
        Has<Grounded>,
        Has<Falling>,
        Has<Jump>,
        Has<DoubleJump>,
        Has<Crouch>,
        Has<HighJump>,
        Has<LongJump>,

        &JumpCounter
    ), (With<CharacterController>, Without<MovementHelper>)>,
    mut commands: Commands,

    time: Res<Time>

) {
    let Ok(camera_transform) = camera.get_single() else {return;};
    let Ok((player_entity, 
        mut player_transform, 
        mut direction,
        is_swinging, 
        is_grounded,
        is_falling,
        is_jumping,
        is_double_jumping,
        is_crouching,
        is_high_jumping,
        is_long_jumping,
        
        jump_counter
    )) = player.get_single_mut() else {return;};

    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        

        // if ratchet is not swinging the wrench it takes the direction input
        if !is_swinging && !is_long_jumping {
            if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {

                let mut vertical = Vec2::ZERO;
                let mut horizontal = Vec2::ZERO;
                if x > 0.3 || x < -0.3 {
                    horizontal =  x * Vec2::new(camera_transform.right().x, -camera_transform.right().z);
                }
    
                if y > 0.3 || y < -0.3 {
                    vertical = y * Vec2::new(camera_transform.forward().x, -camera_transform.forward().z);
                }

                direction.0 = horizontal + vertical;

            }
        }
        
        
        // moves the player
        if direction.0 == Vec2::ZERO {
            commands.entity(player_entity).insert(Idle);
            commands.entity(player_entity).remove::<Walking>();

            if is_swinging {
                let swing_direction = Vec2::new(
                    -player_transform.forward().x,
                    player_transform.forward().z
                ).normalize();
                
                movement_event_writer.send(MovementAction::Swing1(swing_direction / 4.));
            }
        }
        else {
            
            let look_at = player_transform.translation + Vec3::new(-direction.0.x, 0., direction.0.y);
            player_transform.look_at(look_at, Vec3::Y);
            
            if !is_crouching && !is_long_jumping {
                commands.entity(player_entity).insert(Walking);
                commands.entity(player_entity).remove::<Idle>();
                movement_event_writer.send(MovementAction::Move(direction.0));
            }
        }


        // crouch
        let crouch_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::RightTrigger,
        };
        if buttons.pressed(crouch_button) && is_grounded {
            commands.entity(player_entity).insert(Crouch);
        }
        else {
            commands.entity(player_entity).remove::<Crouch>();
        }

        // wrench swing
        let wrench_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::West,
        };

        if buttons.just_pressed(wrench_button) && !is_swinging {
            
            // inserts the swing state
            commands.entity(player_entity).insert(Swing1{
                swing_time: time.elapsed_seconds()
            });

            // [HITBOX]

            // creates a collider that interacts only with hittable entities
            let hitbox = commands.spawn((
                Collider::cylinder(0.5, 1.2),
                Hitbox,
                Damage(1.)
            )).id(); 

            // positions the collider in front of the player
            let hitbox_position = Position::new(
                player_transform.translation + Vec3::new(
                    -player_transform.forward().x , 
                    player_transform.forward().y, 
                    player_transform.forward().z
                ).normalize() / 2.
            );

            // spawns the collider
            commands.entity(hitbox).insert(hitbox_position);
        }

        // jumps and high jumps
        let jump_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };

        if !is_crouching {
            if buttons.just_pressed(jump_button) {
                if jump_counter.counter == 0. && is_grounded {
                    commands.entity(player_entity).insert(Jump);
                    movement_event_writer.send(MovementAction::Jump);
                }
                else {
                    if jump_counter.counter < 2. && jump_counter.counter > 0.
                    && time.elapsed_seconds() < jump_counter.jump_time + 0.67  {
                        movement_event_writer.send(MovementAction::DoubleJump);
                        commands.entity(player_entity).remove::<Jump>();
                        commands.entity(player_entity).insert(DoubleJump);
                        
                    }
                    
                }
            }
        } else {
            if buttons.just_pressed(jump_button) {
                if direction.0 == Vec2::ZERO {
                    commands.entity(player_entity).insert(HighJump);
                    movement_event_writer.send(MovementAction::HighJump);
                }
                else {
                    commands.entity(player_entity).insert(LongJump);
                    movement_event_writer.send(MovementAction::LongJumpStart);

                }   
            }
            

        }

        println!("{}", direction.0);
        if is_long_jumping {
            movement_event_writer.send(MovementAction::LongJump(direction.0));

        }

        // gliding
        if is_falling && !is_jumping && !is_double_jumping && !is_high_jumping && !is_long_jumping {
            if buttons.pressed(jump_button) {
                commands.entity(player_entity).insert(Gliding);
            }
            else {
                commands.entity(player_entity).remove::<Gliding>();
            }
        }

         
    }
}


// Removes the wrench swing when the animation terminates
fn handle_wrench_swing(
    time: Res<Time>,
    mut commands: Commands,
    mut player: Query<(Entity, &Swing1), (With<CharacterController>, Without<MovementHelper>)>,

) {
    let Ok((player_entity, swing)) = player.get_single_mut() else {return;};

    if time.elapsed_seconds() > swing.swing_time + 0.61 {
        commands.entity(player_entity).remove::<Swing1>();
                
    }
}

// Despawns the hitbox of the wrench swing
fn remove_hitbox(
    mut commands: Commands,
    player_q: Query<Has<Swing1>, With<CharacterController>>,
    hitbox_q: Query<Entity, (With<Hitbox>, Without<CharacterController>)>
) {
    let Ok(is_swinging) = player_q.get_single() else {return;};
    let Ok(hitbox_entity) = hitbox_q.get_single() else {return;};
    
    // if the player is not swinging it removes the hitbox
    if !is_swinging {
        commands.entity(hitbox_entity).despawn();
    }
}


/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
    mut jump_counter: Query<&mut JumpCounter>,
    time: Res<Time>,


) {
    let Ok(mut jump_counter) = jump_counter.get_single_mut() else {return;};
    
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
            

            // waits half a second after a jump before resetting the counter 
            // to avoid the counter being reset on the first jump frame
            if time.elapsed_seconds() > jump_counter.jump_time + 0.3 {
                jump_counter.counter = 0.;
                commands.entity(entity).remove::<Falling>();
                commands.entity(entity).remove::<Gliding>();

            }

        } else {
            commands.entity(entity).remove::<Grounded>();
            commands.entity(entity).remove::<Idle>();
            commands.entity(entity).remove::<Walking>();
            
            // jump animation is also the fall animation
            commands.entity(entity).insert(Falling);
        }
    }
}


fn handle_gliding(
    mut player_q: Query< &mut LinearVelocity, (With<CharacterController>, With<Gliding>, Without<DoubleJump>)>
) {
    let Ok(mut linear_velocity) = player_q.get_single_mut() else {return;};

    linear_velocity.y = -1.25;
    linear_velocity.x *= 0.95;
    linear_velocity.z *= 0.95;
    
}