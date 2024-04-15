use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use bevy_xpbd_3d::{math::*, prelude::*};

use crate::{camera::CameraIdentifier, player::*};


pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_grounded,
                crouch,
                walk,
                strafe,
                sideflips,
                longjump,
                jump,
                doublejump,
                highjump,
                gliding,

                
            ).chain());
    }
}

// identifier for the glide audio player 
#[derive(Component)]
pub struct GlideAudio;

#[derive(Component)]
pub struct Idle;

#[derive(Component)]
pub struct Walk;

#[derive(Component)]
pub struct Strafe;

#[derive(Component)]
pub struct Crouch;

#[derive(Component)]
pub struct SideflipL;

#[derive(Component)]
pub struct SideflipR;

#[derive(Component)]
pub struct Longjump;

#[derive(Component)]
pub struct Highjump;


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
        Has<Crouch>,
        Has<SideflipL>,
        Has<SideflipR>,
        Has<Longjump>,
        Has<Strafe>, 
        Has<Highjump>
    ), With<CharacterController>>,
    camera_angle: Query<&CameraIdentifier>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut commands: Commands,
    mut movement_event: EventWriter<MovementAction>
) {
    let Ok((player, mut direction, mut transform, grounded, crouching, sideflip_l, sideflip_r, longjump, strafe, highjump)) = player.get_single_mut() else {return;};
    
    if sideflip_l || sideflip_r || longjump { return; }
    
    if strafe && !highjump { return; }
   

    let Ok(camera_angle) = camera_angle.get_single() else { return; };

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

                // calculates the angle to slerp the player to
                let controller_axes = Vec2::new(x, y).normalize();

                let controller_angle = get_angle(controller_axes.x, controller_axes.y);


                let target_angle = controller_angle - camera_angle.x;

                let target_rotation = Quat::from_rotation_y(target_angle);

                if !crouching {

                    transform.rotation = transform.rotation.slerp(target_rotation, 0.07);
                        
                    direction.0 = Vec2::new(
                        transform.forward().x, 
                        transform.forward().z
                    ).normalize();
                }
                else {
                    transform.rotation = transform.rotation.slerp(target_rotation, 0.007);
                    
                }
                

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

    if !crouching {
        movement_event.send(MovementAction::Walk(direction.0));

    }

}

fn strafe(
    mut player: Query<(
        Entity, 
        &mut PlayerDirection, 
        &mut Transform, 
        Has<Grounded>,
        Has<Crouch>,
        Has<SideflipL>,
        Has<SideflipR>,
        Has<Highjump>,
        Has<Longjump>,
        Has<Glide>
    ), With<CharacterController>>,
    camera_angle: Query<&CameraIdentifier>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut commands: Commands,
    mut movement_event: EventWriter<MovementAction>
) {
    let Ok((player, mut direction, mut transform, grounded, crouching, sideflip_l, sideflip_r, highjump, longjump, glide)) = player.get_single_mut() else {return;};
    if crouching || sideflip_l || sideflip_r || highjump || longjump || glide {return;}
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

            if !buttons.pressed(strafe) {
                commands.entity(player).remove::<Strafe>();
                return;
            }

            let target_rotation = Quat::from_rotation_y(-camera_angle.x + 1.5708);

            transform.rotation = transform.rotation.slerp(target_rotation, 0.13);

            if x != 0. && !x.is_nan()
            || y != 0. && !y.is_nan() {

                let controller_axes = Vec2::new(x, y).normalize();

                let controller_angle = get_angle(controller_axes.x, controller_axes.y);

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


fn crouch(
    mut player: Query<(
        Entity, 
        Has<Grounded>
    ), With<CharacterController>>,
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut commands: Commands
) {
    let Ok((player_entity, grounded)) = player.get_single_mut() else {return;};
    for gamepad in gamepads.iter() {
        let crouch = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::RightTrigger
        };


        if buttons.pressed(crouch) && grounded{
            commands.entity(player_entity).insert(Crouch);
            commands.entity(player_entity).remove::<Land>();

        }
        else {
            commands.entity(player_entity).remove::<Crouch>();

        }
    }



}

fn sideflips(
    mut player: Query<(
        Entity,
        &mut PlayerDirection,
        Has<Crouch>, 
        &Transform,
        Has<Strafe>,
        Has<Grounded>,
        &mut LinearVelocity,
        &mut JumpCounter
    ), With<CharacterController>>,
    camera_angle: Query<&CameraIdentifier>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut commands: Commands,
    mut movement_event: EventWriter<MovementAction>,
    asset_server: Res<AssetServer>   

) {
    let Ok((entity, direction, crouching, transform, strafing, grounded, mut velocity, mut jump_counter, )) = player.get_single_mut() else {return;};
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

            if x < 0.2 && x > -0.2 || y < 0.2 && y > -0.2 {return;}

            let controller_axes = Vec2::new(x, y).normalize();

            // sideflip phisycs chage while strafing
            if !strafing {

                // non-strafing sideflip

                let controller_angle = get_angle(controller_axes.x, controller_axes.y).to_degrees();

                let mut player_angle = get_angle(-direction.0.x, direction.0.y).to_degrees() + 90.;

                player_angle += camera_angle.x.to_degrees();


                // i don't have the slightest idea of why player_angle sometimes skyrockets beyond 720, however this solution should work. 
                if player_angle > 720. {
                    player_angle -= 720.;
                }
                else if player_angle > 360. {
                    player_angle -= 360.;
                }
                

                let jump = GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::South
                };

                if crouching && buttons.just_pressed(jump) {
                    if player_angle - controller_angle + camera_angle.x > 25. {
                        commands.entity(entity).insert(SideflipR);
                        movement_event.send(MovementAction::Sideflip(Vec2::new(
                            transform.right().x, 
                            transform.right().z
                        )));

                        jump_counter.counter = 2.;

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
                    else if player_angle - controller_angle + camera_angle.x < -25. {
                        
                        
                        commands.entity(entity).insert(SideflipL);
                        movement_event.send(MovementAction::Sideflip(Vec2::new(
                            transform.left().x, 
                            transform.left().z
                        )));

                        jump_counter.counter = 2.;

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
            else {

                // strafing sideflip 

                let jump = GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::South
                };

                if buttons.just_pressed(jump) {
                    if controller_axes.normalize().x > 0.4226 && grounded {
                        println!("aaa");
                        velocity.x = 0.;
                        velocity.z = 0.;
                        commands.entity(entity).insert(SideflipR);
                        movement_event.send(MovementAction::Sideflip(Vec2::new(
                            transform.right().x, 
                            transform.right().z
                        )));

                        jump_counter.counter = 2.;

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
                    else if controller_axes.normalize().x < -0.4226 && grounded {
                        velocity.x = 0.;
                        velocity.z = 0.;
                        commands.entity(entity).insert(SideflipL);
                        movement_event.send(MovementAction::Sideflip(Vec2::new(
                            transform.left().x, 
                            transform.left().z
                        ).normalize()));

                        jump_counter.counter = 2.;

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

        }
    }
}

fn longjump(
    mut player: Query<(
        Entity,
        &mut PlayerDirection,
        Has<Crouch>, 
        &Transform,
        Has<Longjump>,
        Has<Highjump>,
        &LinearVelocity,
        &mut JumpCounter,
        &mut GravityScale
    ), With<CharacterController>>,
    camera_angle: Query<&CameraIdentifier>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut commands: Commands,
    mut movement_event: EventWriter<MovementAction>,
    asset_server: Res<AssetServer> ,
    time: Res<Time>  

) {
    let Ok((entity, mut direction, crouching, transform, longjumping, highjumping,velocity, mut jump_counter, mut gravity_scale)) = player.get_single_mut() else {return;};
    if highjumping {return;}

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

            let controller_axes = Vec2::new(x, y).normalize();

            let controller_angle = get_angle(controller_axes.x, controller_axes.y).to_degrees();

            let mut player_angle = get_angle(-direction.0.x, direction.0.y).to_degrees() + 90.;

            player_angle += camera_angle.x.to_degrees();


            // i don't have the slightest idea of why player_angle sometimes skyrockets beyond 720, however this solution should work. 
            if player_angle > 720. {
                player_angle -= 720.;
            }
            else if player_angle > 360. {
                player_angle -= 360.;
            }
            

            let jump = GamepadButton {
                gamepad,
                button_type: GamepadButtonType::South
            };




            if crouching && buttons.just_pressed(jump) && velocity.length() > 2. {
                if player_angle - controller_angle + camera_angle.x < 25. 
                && player_angle - controller_angle + camera_angle.x > -25. {

                    commands.entity(entity).insert(Longjump);

                    movement_event.send(MovementAction::Longjump(direction.0));                        


                    jump_counter.counter = 2.;
                    jump_counter.jump_time = time.elapsed_seconds();

                    commands.spawn((
                        AudioBundle {
                            source: asset_server.load("glide.ogg"),
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


            if longjumping {
                direction.0 = Vec2::new(transform.forward().x, transform.forward().z);

                if time.elapsed_seconds() > jump_counter.jump_time + 1.45 {
                    commands.entity(entity).remove::<Longjump>();

                }
            }
            else {
                gravity_scale.0 = 3.;
            }

        }
    }
}

fn highjump(
    mut player: Query<(
        Entity,
        Has<Crouch>, 
        Has<SideflipL>, 
        Has<SideflipR>, 
        Has<Highjump>,
        Has<Longjump>,
        &LinearVelocity,
        &mut JumpCounter,
        &mut GravityScale
    ), With<CharacterController>>,
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut commands: Commands,
    mut movement_event: EventWriter<MovementAction>,
    asset_server: Res<AssetServer> ,
    time: Res<Time>  

) {
    let Ok((entity, crouching, sideflip_l, sideflip_r, highjumping, longjumping, velocity, mut jump_counter, mut gravity_scale)) = player.get_single_mut() else {return;};
    if longjumping || sideflip_l || sideflip_r {return;}

 
    for gamepad in gamepads.iter() {

        let jump = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South
        };


        if crouching && buttons.just_pressed(jump) && velocity.length() < 2. {
        
            commands.entity(entity).insert(Highjump);

            movement_event.send(MovementAction::Highjump1);                        


            jump_counter.counter = 2.;
            jump_counter.jump_time = time.elapsed_seconds();

            commands.spawn((
                AudioBundle {
                    source: asset_server.load("glide.ogg"),
                    settings: PlaybackSettings {
                        volume: Volume::new(0.07),
                        speed: 0.75,
                        mode: PlaybackMode::Despawn,
                        ..default()
                    }
                },
            ));

                
        }

        if highjumping {

            if time.elapsed_seconds() > jump_counter.jump_time + 1.42 {
                commands.entity(entity).remove::<Highjump>();


            }
            else if time.elapsed_seconds() > jump_counter.jump_time + 0.7 {
                movement_event.send(MovementAction::Highjump2);
                gravity_scale.0 = 1.5;

            }

        }
        else {
            gravity_scale.0 = 3.;
        }
    }
}

fn jump(
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut player: Query<(Entity, &JumpCounter, Has<Grounded>, Has<Crouch>), With<CharacterController>>,
    mut movement_event: EventWriter<MovementAction>,
    mut commands: Commands,
    
    asset_server: Res<AssetServer>   

) {
    let Ok((player, jump_counter, grounded, crouch,)) = player.get_single_mut() else {return;};

    if crouch {return;};

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

fn gliding(
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut player: Query<(
        Entity, 
        Has<Jump>, 
        Has<DoubleJump>, 
        Has<Glide>, 
        Has<Grounded>,
        Has<SideflipL>,
        Has<SideflipR>,
        Has<Longjump>,
        Has<Highjump>
    ), With<CharacterController>>,
    mut movement_event: EventWriter<MovementAction>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    glide_audio: Query<Entity, With<GlideAudio>>,


) {
    let Ok((player, jumping, doublejumping, gliding, grounded, sideflip_l, sideflip_r, longjumping, highjumping)) = player.get_single_mut() else {return;};
    for gamepad in gamepads.iter() {
        let glide = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South
        };

        if !jumping && !doublejumping && !sideflip_l && !sideflip_r && !grounded && !longjumping && !highjumping {
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
    let Ok((entity, hits, rotation, max_slope_angle, is_falling, mut jump_counter,)) = query.get_single_mut() else {return;};


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
            commands.entity(entity).remove::<SideflipL>();
            commands.entity(entity).remove::<SideflipR>();

            commands.entity(entity).insert(Land);
            commands.entity(entity).remove::<Longjump>();


            jump_counter.counter = 0.;

        }
        

        
        
    }
    else {
        commands.entity(entity).remove::<Grounded>();
        commands.entity(entity).insert(Falling);
    }
    
}










fn get_angle(cos: f32, sin: f32) -> f32 {
    let mut angle = cos.acos();

    if sin < 0. {
        angle *= -1. ;
    } 

    if angle < 0. {
        angle += 6.28319;
    }

    return angle;
}