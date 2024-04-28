use bevy::prelude::*;

use crate::{player::{CameraTarget, GroundedHeight}, player_input::{DoubleJump, Jump, Longjump, SideflipL, SideflipR}};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                camera_rotate,
                camera_height,
                camera_position,
                camera_rotation
            ).chain());
    }
}

#[derive(Component)]
pub struct CameraIdentifier(pub f32);

#[derive(Component)]
pub struct CameraHeight(pub f32);

#[derive(Component)]
pub struct CameraRealHeight(pub f32);

// if the camera is lerping
#[derive(Component)]
pub struct Lerping;



fn spawn_camera(
    mut commands: Commands,
) {

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5., 5., -5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraIdentifier(0.),
        CameraHeight(0.),
        CameraRealHeight(0.),

        // RigidBody::Dynamic,
        // Collider::capsule(0.2, 0.2),
        // GravityScale(0.0),
        // Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),

        
    ));
}

const CAMERA_DISTANCE: f32 = 5.;
const CAMERA_HEIGHT_SPEED: f32 = 12.;

fn camera_rotate(
    mut camera: Query<&mut CameraIdentifier>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    time: Res<Time>
) {
    let Ok(mut camera_angle) = camera.get_single_mut() else {return;};

    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::RightStickX
        };

        if let Some(x) = axes.get(axis_lx) {
            camera_angle.0 += x * time.delta_seconds() * 2.;
        }
    }

    if camera_angle.0 > 6.28319 {
        camera_angle.0 = 0.;
    }
    else if camera_angle.0 < 0. {
        camera_angle.0 += 6.28319;

    }

}

fn camera_height (
    mut camera: Query<(&mut CameraHeight, &mut CameraRealHeight)>,
    target: Query<(
        Has<DoubleJump>, 
        Has<Jump>, 
        Has<SideflipL>, 
        Has<SideflipR>, 
        Has<Longjump>, 
        &Transform,
        &GroundedHeight
    ), (With<CameraTarget>, Without<CameraRealHeight>)>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    time: Res<Time>,
) {
    let Ok((mut camera_height, mut real_camera_height)) = camera.get_single_mut() else {return;};
    let Ok((target_doublejump, target_jump, target_sideflip_l, target_sideflip_r, target_longjump, player_transform, grounded_height)) = target.get_single() else {return;};

    for gamepad in gamepads.iter() {
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::RightStickY
        };

        if let Some(y) = axes.get(axis_ly) {

            // if the stick is neutral it gradually puts back the camera to 0  
            if y < 0.2 && y > -0.2 {
                camera_height.0 = lerp(camera_height.0, 0., 3., &time);
            }
            else {
                if camera_height.0 < 4. && camera_height.0 > -1.5 {
                    camera_height.0 += y * time.delta_seconds() * 5.;
                }
            }
        }

    }

    if !target_doublejump
    && !target_jump
    && !target_sideflip_l
    && !target_sideflip_r
    && !target_longjump {
        real_camera_height.0 = lerp(
            real_camera_height.0,
                player_transform.translation.y + 1. + camera_height.0, 
                CAMERA_HEIGHT_SPEED,
                &time
        );
        
    }
    else {
        real_camera_height.0 = lerp(
            real_camera_height.0,
                grounded_height.0 + 1. + camera_height.0, 
                CAMERA_HEIGHT_SPEED,
                &time
        );
    }

}


fn lerp(
    mut camera_height: f32,
    target_value: f32,
    speed: f32,
    time: &Res<Time>
) -> f32 {
    if camera_height > target_value {
        if camera_height - time.delta_seconds() * speed > target_value {
            camera_height -= time.delta_seconds() * speed;
        } 
        else {
            camera_height = target_value;
        }
    }
    else {
        if camera_height + time.delta_seconds() * speed < target_value {
            camera_height += time.delta_seconds() * speed;
        } 
        else {
            camera_height = target_value;
        }
    }
    camera_height
}



fn camera_position (
    mut camera: Query<(&mut Transform, &mut CameraRealHeight, &CameraIdentifier)>,
    target: Query<&Transform , (With<CameraTarget>, Without<CameraRealHeight>)>,

) {

    let Ok((mut camera_transform, camera_height, camera_angle)) = camera.get_single_mut() else {return;};
    let Ok(target_transform) = target.get_single() else {return;};
    
    // calculates the camera position through trigonometry and adds the player position to it
    let x = target_transform.translation.x + CAMERA_DISTANCE * f32::cos(camera_angle.0); 
    let z = target_transform.translation.z + CAMERA_DISTANCE * f32::sin(camera_angle.0);
    let y =  camera_height.0; 


    
    camera_transform.translation = camera_transform.translation.lerp(
        Vec3::new(x, y, z),
        0.07
    );
            
    

}


fn camera_rotation(
    mut camera: Query<&mut Transform, With<CameraIdentifier>>,
    target: Query<(
        &Transform,
        &GroundedHeight,
        Has<DoubleJump>, 
        Has<Jump>, 
        Has<SideflipL>, 
        Has<SideflipR>, 
        Has<Longjump>
    ), (With<CameraTarget>, Without<CameraIdentifier>)>,

) {
    let Ok((target_transform, grounded_height, target_doublejump, target_jump, target_sideflip_l, target_sideflip_r, target_longjump)) = target.get_single() else {return;};
    let Ok(mut camera_transform) = camera.get_single_mut() else {return;};
   
    if !target_doublejump
    && !target_jump
    && !target_sideflip_l
    && !target_sideflip_r
    && !target_longjump {
        let target_rotation = camera_transform.looking_at(Vec3::new(
            target_transform.translation.x, 
            target_transform.translation.y + 0.5,
            target_transform.translation.z
        ), Vec3::Y).rotation;
    
        camera_transform.rotation = camera_transform.rotation.slerp(
            target_rotation,
            0.15
        );
    }
    else {
        let target_rotation = camera_transform.looking_at(Vec3::new(
            target_transform.translation.x, 
            grounded_height.0 + 0.5,
            target_transform.translation.z
        ), Vec3::Y).rotation;
    
        camera_transform.rotation = camera_transform.rotation.slerp(
            target_rotation,
            0.15
        );
    }
}