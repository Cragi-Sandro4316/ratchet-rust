use bevy::prelude::*;
use bevy_xpbd_3d::components::{GravityScale, LinearVelocity};

use crate::{player::*, player_input::*};

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
        &mut JumpCounter,
        &mut GravityScale
    ), With<CharacterController>>
) {
    let Ok((
        movement_acceleration,
        jump_impulse,
        double_jump_impulse,
        mut linear_velocity,
        mut jump_counter,
        mut gravity_scale
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
                linear_velocity.x *= 0.90;
                linear_velocity.z *= 0.90;
            }
            MovementAction::Sideflip(direction) => {
                linear_velocity.x += linear_velocity.x / 1.8 + direction.x * 4.;
                linear_velocity.z += linear_velocity.z / 1.8 + direction.y * 4.;
                linear_velocity.y = 12.2;

            }
            MovementAction::Longjump(direction) => {
                
                gravity_scale.0 = 1.;

                linear_velocity.y = 5.5;
                linear_velocity.x = direction.normalize().x * 14.7;
                linear_velocity.z = direction.normalize().y * 14.7;

            }
            MovementAction::Highjump1 => {
                gravity_scale.0 = 1.3;
                linear_velocity.y = 6.5;

            }
            MovementAction::Highjump2 => {
                linear_velocity.y = 3.8;
            }
            MovementAction::Swing(direction) => {
                let normalized_direction = direction.normalize();

                linear_velocity.x += normalized_direction.x * 5.;
                linear_velocity.z += normalized_direction.y * 5.;

            }
        }
    }



}




fn damp_movement(
    mut player: Query<(
        &MovementDampingFactor, 
        &mut LinearVelocity, 
        Has<Grounded>,
        Has<SideflipL>,
        Has<SideflipR>,
        Has<Longjump>,
        Has<Highjump>
    ), With<CharacterController>>,

) {
    let Ok((damping_factor, mut linear_velocity, grounded, sideflip_l, sideflip_r, longjump, highjump)) = player.get_single_mut() else {return;};

    if grounded {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
    else if highjump {
        linear_velocity.x *= 0.85;
        linear_velocity.z *= 0.85;
    }
    else if !sideflip_l && !sideflip_r && !longjump {
        linear_velocity.x *= 0.927;
        linear_velocity.z *= 0.927;
    }
    

}