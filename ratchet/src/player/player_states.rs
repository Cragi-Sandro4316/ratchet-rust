
use bevy::prelude::*;

use crate::{player::*, camera::MovementHelper, platform::Layer};
use bevy_xpbd_3d::{math::*, prelude::*};

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
                update_grounded,
                keyboard_input,
                gamepad_input,
                handle_wrench_swing,
                remove_hitbox
            ).chain()
        );
    }
}


#[derive(Component)]
pub struct Jump;

#[derive(Component)]
pub struct Walking;

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


/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
    camera: Query<&Transform, With<MovementHelper>>,
    mut player: Query<(Entity, &mut Transform), (With<CharacterController>, Without<MovementHelper>)>,
    mut commands: Commands

) {
    let Ok(camera_transform) = camera.get_single() else {return;};
    let Ok((player_entity, mut player_transform)) = player.get_single_mut() else {return;};

    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

    let vertical = (up as i8 - down as i8) as f32 * Vec2::new(camera_transform.forward().x, -camera_transform.forward().z);
    let horizontal =  (right as i8 - left as i8) as f32 * Vec2::new(camera_transform.right().x, -camera_transform.right().z);

    let direction = horizontal + vertical;
   

    movement_event_writer.send(MovementAction::Move(direction));

        if direction == Vec2::ZERO {
            commands.entity(player_entity).insert(Idle);
            commands.entity(player_entity).remove::<Walking>();
        }
        else {
            let look_at = player_transform.translation + Vec3::new(-direction.x, 0., direction.y);
            player_transform.look_at(look_at, Vec3::Y);
            commands.entity(player_entity).insert(Walking);
            commands.entity(player_entity).remove::<Idle>();
        }

    if keyboard_input.any_just_pressed([KeyCode::Space]) {
        movement_event_writer.send(MovementAction::Jump);
        commands.entity(player_entity).insert(Jump);
        
    }
}




/// Sends [`MovementAction`] events based on gamepad input.
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    camera: Query<&Transform, With<MovementHelper>>,
    mut player: Query<(Entity, &mut Transform, Has<Swing1>), (With<CharacterController>, Without<MovementHelper>)>,
    mut commands: Commands,

    time: Res<Time>

) {
    let Ok(camera_transform) = camera.get_single() else {return;};
    let Ok((player_entity, mut player_transform, is_swinging)) = player.get_single_mut() else {return;};

    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        let mut direction: Vec2 = Vec2::ZERO;

        // if ratchet is not swinging the wrench it takes the direction input
        if !is_swinging {
            if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
                let vertical = y * Vec2::new(camera_transform.forward().x, -camera_transform.forward().z);
                let horizontal =  x * Vec2::new(camera_transform.right().x, -camera_transform.right().z);
    
                direction = horizontal + vertical;
            }
        }
        
        
        
        if direction == Vec2::ZERO {
            commands.entity(player_entity).insert(Idle);
            commands.entity(player_entity).remove::<Walking>();

            if is_swinging {
                let swing_direction = Vec2::new(
                    -player_transform.forward().x,
                    player_transform.forward().z
                ).normalize();
                
                movement_event_writer.send(MovementAction::Move(swing_direction / 4.));
            }
        }
        else {
            
            let look_at = player_transform.translation + Vec3::new(-direction.x, 0., direction.y);
            player_transform.look_at(look_at, Vec3::Y);
            commands.entity(player_entity).insert(Walking);
            commands.entity(player_entity).remove::<Idle>();
            movement_event_writer.send(MovementAction::Swing1(direction));
        
        }




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
                CollisionLayers::new([Layer::Player], [Layer::Hittable]),
                Hitbox,
                Damage(1.)
            )).id(); 

            // positions the collider in front of the player
            let hitbox_position = Position::new(
                player_transform.translation + player_transform.forward().normalize() / 2.
            );

            // spawns the collider
            commands.entity(hitbox).insert(hitbox_position);
        }


        let jump_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };

        if buttons.just_pressed(jump_button) {
            movement_event_writer.send(MovementAction::Jump);
            commands.entity(player_entity).insert(Jump);
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
        //commands.entity(hitbox_entity).despawn();
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
            commands.entity(entity).remove::<Jump>();

            // waits half a second after a jump before resetting the counter 
            // to avoid the counter being reset on the first jump frame
            if time.elapsed_seconds() > jump_counter.jump_time + 0.5 {
                jump_counter.counter = 0.;
            }

        } else {
            commands.entity(entity).remove::<Grounded>();
            
            // jump animation is also the fall animation
            commands.entity(entity).insert(Jump);
        }
    }
}
