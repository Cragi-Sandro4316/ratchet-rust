use bevy::prelude::*;
use bevy_xpbd_3d::{components::LinearVelocity, math::AdjustPrecision};

use crate::{player::*, player_states::{LongJump, SideFlip}};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            movement,
            apply_movement_damping
        ).chain());
    }
}


/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &DoubleJumpImpulse,
        &mut LinearVelocity,
        &mut JumpCounter,
    )>,


) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, double_jump_impulse, mut linear_velocity, mut jump_counter) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.z -= direction.y * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    jump_counter.jump_time = time.elapsed_seconds();
                    linear_velocity.y = jump_impulse.0;
                    jump_counter.counter += 1.;
                        
                        
                }
                MovementAction::DoubleJump => {
                    linear_velocity.y = double_jump_impulse.0;
                    jump_counter.counter += 1.;

                }
                MovementAction::Swing1(direction) => {
                    linear_velocity.x = direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.z = -direction.y * movement_acceleration.0 * delta_time;
                
                }
                MovementAction::HighJump => {
                    linear_velocity.y = 16.;
                }
                MovementAction::LongJump => {
                    
                    linear_velocity.x = linear_velocity.x * 1.087;
                    linear_velocity.z = linear_velocity.z * 1.087;
                    

                }
                MovementAction::LongJumpStart(direction) => {
                    jump_counter.jump_time = time.elapsed_seconds();
                    linear_velocity.y = 12.;

                    

                    if *direction != Vec2::ZERO {
                        linear_velocity.x += direction.x * 11.5;
                        linear_velocity.z -= direction.y * 11.5;
                    }
                    else {
                        linear_velocity.x += linear_velocity.x * 2.;
                        linear_velocity.z += linear_velocity.z * 2.;
                    }
                    
                }
                MovementAction::SideFlip(direction) => {
                    linear_velocity.y = 12.;
                    linear_velocity.x +=  linear_velocity.x * 1.3 + direction.x * 4.;
                    linear_velocity.z +=  linear_velocity.z * 1.3 + direction.y * 4.;

                }
            }

        }

    }
}


/// Slows down movement in the XZ plane.
fn apply_movement_damping(
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>,
    player: Query<
        (
            Has<Grounded>,
            Has<LongJump>,
            Has<SideFlip>
        ), 
        With<CharacterController>
    >
) {
    let Ok((is_grounded, is_longjumping, is_sideflipping)) = player.get_single() else {return;};
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        
       
        if is_grounded || is_longjumping {
            linear_velocity.x *= damping_factor.0;
            linear_velocity.z *= damping_factor.0;
        }
        else if !is_sideflipping {            
            linear_velocity.x *= 0.94 ;
            linear_velocity.z *= 0.94 ;
        }
    
    }

}
