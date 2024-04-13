use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use bevy_xpbd_3d::{math::*, prelude::*};

use crate::{camera::CameraIdentifier, player::*};


pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_grounded,
                walk,
                strafe,
                jump,
                doublejump,
                gliding,
                print_stuff
            ).chain());
    }
}


#[derive(Component)]
pub struct Idle;

#[derive(Component)]
pub struct Walk;

#[derive(Component)]
pub struct Strafe;

#[derive(Component)]
pub struct Jump;

#[derive(Component)]
pub struct DoubleJump;

#[derive(Component)]
pub struct Glide;

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct Falling;

#[derive(Component)]
pub struct Land;

fn walk(
    mut player: Query<(
        Entity, 
        &mut PlayerDirection, 
        &mut Transform, 
        Has<Grounded>,
    ), With<CharacterController>>,
    camera_angle: Query<&CameraIdentifier>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut commands: Commands,
    mut movemnt_event: EventWriter<MovementAction>
) {
    let Ok((player, mut direction, mut transform, grounded)) = player.get_single_mut() else {return;};
    let Ok(camera_angle) = camera_angle.get_single() else {return;};

    for gamepad in gamepads.iter() {
        let axis_rx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX
        };
    
        let axis_ry = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY
        };



        if let(Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
            
            if x > 0.2 || x < -0.2 && !x.is_nan()
            || y > 0.2 || y < -0.2 && !y.is_nan() {

                let strafe = GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::LeftTrigger2
                };
    
                if buttons.pressed(strafe) {return;}


                let controller_axes = Vec2::new(x, y).normalize();

                let controller_angle = controller_stick_angle(controller_axes.x, controller_axes.y);

                let target_angle = -camera_angle.x + controller_angle;

                let target_rotation = Quat::from_rotation_y(target_angle);

                transform.rotation = transform.rotation.slerp(target_rotation, 0.13);


                direction.0 = Vec2::new(
                    transform.forward().x, 
                    transform.forward().z
                ).normalize();

                commands.entity(player).remove::<Land>();
                commands.entity(player).insert(Walk);

                if grounded {
                    commands.entity(player).remove::<DoubleJump>();
                }
                
            }
            else {
                direction.0 = Vec2::new(
                    0.,
                    0.
                );
                commands.entity(player).insert(Idle);
                commands.entity(player).remove::<Walk>();

                if grounded {
                    commands.entity(player).remove::<DoubleJump>();
                }
                
            }
        }
        

    }

    movemnt_event.send(MovementAction::Walk(direction.0));

}


fn strafe(
    mut player: Query<(
        Entity, 
        &mut PlayerDirection, 
        &mut Transform, 
        Has<Grounded>,
    ), With<CharacterController>>,
    camera_angle: Query<&CameraIdentifier>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut commands: Commands,
    mut movement_event: EventWriter<MovementAction>
) {
    let Ok((player, mut direction, mut transform, grounded)) = player.get_single_mut() else {return;};
    let Ok(camera_angle) = camera_angle.get_single() else {return;};


    for gamepad in gamepads.iter() {
        let axis_rx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX
        };
    
        let axis_ry = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY
        };



        if let(Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
            
            let strafe = GamepadButton {
                gamepad,
                button_type: GamepadButtonType::LeftTrigger2
            };

            if !buttons.pressed(strafe) {return;}

            let target_rotation = Quat::from_rotation_y(-camera_angle.x + 1.5708);

            transform.rotation = transform.rotation.slerp(target_rotation, 0.13);

            if x != 0. && !x.is_nan()
            || y != 0. && !y.is_nan() {

                let controller_axes = Vec2::new(x, y).normalize();

                let controller_angle = controller_stick_angle(controller_axes.x, controller_axes.y);

                let direction_angle = -camera_angle.x + controller_angle;


                direction.0 = Vec2::new(
                    -direction_angle.sin(), 
                    -direction_angle.cos(), 
                    
                );


                commands.entity(player).insert(Strafe);
                commands.entity(player).remove::<Land>();
                commands.entity(player).insert(Walk);

                if grounded {
                    commands.entity(player).remove::<DoubleJump>();
                }

            }
            else {
                direction.0 = Vec2::new(0., 0.);
                commands.entity(player).insert(Idle);
                commands.entity(player).remove::<Walk>();
                
                if grounded {
                    commands.entity(player).remove::<DoubleJump>();
                }
            }
        }
    }

    movement_event.send(MovementAction::Walk(direction.0));

}



fn jump(
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut player: Query<(Entity, &JumpCounter, Has<Grounded>), With<CharacterController>>,
    mut movement_event: EventWriter<MovementAction>,
    mut commands: Commands,
    
    asset_server: Res<AssetServer>   

) {
    let Ok((player, jump_counter, grounded)) = player.get_single_mut() else {return;};

    for gamepad in gamepads.iter() {
        let jump = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South
        };


        if buttons.just_pressed(jump) && jump_counter.counter < 1. && grounded {
            commands.entity(player).insert(Jump);
            commands.entity(player).remove::<Walk>();
            movement_event.send(MovementAction::Jump);


            // spawns an audio player that plays the jump sound
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("jump.ogg"),
                    settings: PlaybackSettings {
                        volume: Volume::new(0.07),
                        speed: 0.75,
                        mode: PlaybackMode::Despawn,
                        ..default()
                    }
                },
                
            ));

        }
    }

}

fn doublejump(
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut player: Query<(Entity, &JumpCounter), With<CharacterController>>,
    mut movement_event: EventWriter<MovementAction>,
    mut commands: Commands,
    time: Res<Time>,

    asset_server: Res<AssetServer>   

) {
    let Ok((player, jump_counter)) = player.get_single_mut() else {return;};

    for gamepad in gamepads.iter() {
        let jump = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South
        };


        if buttons.just_pressed(jump) 
        && jump_counter.counter < 2.
        && jump_counter.counter > 0.
        && jump_counter.jump_time + 0.65 > time.elapsed_seconds() {
            commands.entity(player).remove::<Jump>();
            commands.entity(player).insert(DoubleJump);

            movement_event.send(MovementAction::DoubleJump);

            // spawns an audio player that plays the doublejump sound
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("jump2.ogg"),
                    settings: PlaybackSettings {
                        volume: Volume::new(0.07),
                        speed: 0.75,
                        mode: PlaybackMode::Despawn,
                        ..default()
                    }
                },
            ));

        }
    }

}


#[derive(Component)]
pub struct GlideAudio;

fn gliding(
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut player: Query<(Entity, Has<Jump>, Has<DoubleJump>, Has<Glide>, Has<Grounded>), With<CharacterController>>,
    mut movement_event: EventWriter<MovementAction>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    glide_audio: Query<Entity, With<GlideAudio>>,


) {
    let Ok((player, jumping, doublejumping, gliding, grounded)) = player.get_single_mut() else {return;};
    for gamepad in gamepads.iter() {
        let glide = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South
        };

        if !jumping && !doublejumping && !grounded {
            if buttons.pressed(glide)  {
                commands.entity(player).insert(Glide);
                movement_event.send(MovementAction::Gliding);

                // if ratchet just started gliding it plays the glide audio
                if !gliding {
                    commands.spawn((
                        AudioBundle {
                            source: asset_server.load("glide.ogg"),
                            settings: PlaybackSettings {
                                volume: Volume::new(0.07),
                                speed: 0.75,
                                mode: PlaybackMode::Loop,
                                ..default()
                            }
                        },
                        GlideAudio
                    ));

                }
            }
            else {
                commands.entity(player).remove::<Glide>();
            }
        }
        

        if !gliding {
            for audio in glide_audio.iter() {
                commands.entity(audio).despawn();
            }
        }
    }
}


fn update_grounded(
    mut commands: Commands,
    mut query: Query<(
            Entity, 
            &ShapeHits, 
            &Rotation, 
            Option<&MaxSlopeAngle>,
            Has<Falling>,
            &mut JumpCounter,
        ),
        With<CharacterController>,
    >,



) {
    let Ok((entity, hits, rotation, max_slope_angle, is_falling, mut jump_counter)) = query.get_single_mut() else {return;};


    // if the ground check detects a hit it checks the slope angle of the mesh it has just hit
    // if it's too steep then the character will not be grounded
    let is_grounded = hits.iter().any(|hit| {
        if let Some(angle) = max_slope_angle {
            rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle.0
        } else {
            true
        }
    });

    if is_grounded {
        commands.entity(entity).remove::<Falling>();
        commands.entity(entity).remove::<Glide>();
        commands.entity(entity).insert(Grounded);

        // if ratchet was falling and now he's grounded he just landed
        if is_falling {
            commands.entity(entity).insert(Land);
            jump_counter.counter = 0.;

        }

        
        
    }
    else {
        commands.entity(entity).remove::<Grounded>();
        commands.entity(entity).insert(Falling);
    }
    
}








fn print_stuff(
    //querty: Query<(Has<Grounded>, Has<Falling>), With<CharacterController>>
) {
    //let Ok((is_grounded, is_falling)) = querty.get_single() else {return;};


}





fn controller_stick_angle(cos: f32, sin: f32) -> f32 {
    let mut angle = cos.acos();

    if sin < 0. {
        angle *= -1.;
    } 

    return angle;
}