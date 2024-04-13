use bevy::prelude::*;
use bevy_xpbd_3d::components::LinearVelocity;

use crate::player::*;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_systems(Update, (
                movement,
                damp_movement
            ));
    }
}

fn movement(
    time: Res<Time>,
    mut movement_event: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &DoubleJumpImpulse,
        &mut LinearVelocity,
        &mut JumpCounter
    ), With<CharacterController>>
) {
    let Ok((
        movement_acceleration,
        jump_impulse,
        double_jump_impulse,
        mut linear_velocity,
        mut jump_counter
    )) = controllers.get_single_mut() else {return;};

    for action in movement_event.read() {
        match action {
            MovementAction::Walk(direction) => {
                linear_velocity.x += direction.x * movement_acceleration.0 * time.delta_seconds();
                linear_velocity.z += direction.y * movement_acceleration.0 * time.delta_seconds();

            }
            MovementAction::Jump => {
                jump_counter.jump_time = time.elapsed_seconds();
                linear_velocity.y = jump_impulse.0;
                jump_counter.counter += 1.;
            }
            MovementAction::DoubleJump => {
                jump_counter.jump_time = time.elapsed_seconds();
                linear_velocity.y = double_jump_impulse.0;
                jump_counter.counter += 1.;
            }
            MovementAction::Gliding => {
                linear_velocity.y = -1.35;
                linear_velocity.x *= 0.95;
                linear_velocity.z *= 0.95;
            }
        }
    }

}




fn damp_movement(
    mut player: Query<(&MovementDampingFactor, &mut LinearVelocity), With<CharacterController>>,

) {
    let Ok((damping_factor, mut linear_velocity)) = player.get_single_mut() else {return;};

    linear_velocity.x *= damping_factor.0;
    linear_velocity.z *= damping_factor.0;

}