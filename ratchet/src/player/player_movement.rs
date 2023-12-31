use bevy::prelude::*;
use bevy_xpbd_3d::{components::LinearVelocity, math::AdjustPrecision};

use crate::player::*;

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
        Has<Grounded>,
        &mut JumpCounter,
    )>,


) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, double_jump_impulse, mut linear_velocity, is_grounded, mut jump_counter) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.z -= direction.y * movement_acceleration.0 * delta_time;
                
                }
                MovementAction::Jump => {
                    if is_grounded {
                        jump_counter.jump_time = time.elapsed_seconds();
                        linear_velocity.y = jump_impulse.0;
                        jump_counter.counter += 1.;
                    } 
                    else if jump_counter.counter < 2. 
                    && jump_counter.counter > 0. 
                    && time.elapsed_seconds() < jump_counter.jump_time + 0.85 {
                        linear_velocity.y = double_jump_impulse.0;
                        jump_counter.counter += 1.;
                        
                    }
                }
                MovementAction::Swing1(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.z -= direction.y * movement_acceleration.0 * delta_time;
                }

            }

        }

    }
}


/// Slows down movement in the XZ plane.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}
