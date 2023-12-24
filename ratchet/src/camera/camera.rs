use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_third_person_camera::*;

use crate::player::CameraTarget;

pub struct CameraPlugin_;

impl Plugin for CameraPlugin_ {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                update_camera_pos, 
                rotate_camera_horizontal, 
                
            ));
    }
}

#[derive(Component)]
pub struct CameraIdentifier{
    pub x: f32,
    pub y: f32
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            zoom: Zoom::new(10., 10.),
            ..default()
        },
        CameraIdentifier {
            x: 0.,
            y: 0.
        }
    ));
}

const CAMERA_DISTANCE: f32 = 10.;

fn update_camera_pos(
    mut camera: Query<&mut Transform, With<CameraIdentifier>>,
    mut player: Query<&Transform, (With<CameraTarget>, Without<CameraIdentifier>)>

) {
    let Ok(mut camera_transform) = camera.get_single_mut() else {return;};
    let Ok(player_transform) = player.get_single_mut() else {return;};
    camera_transform.rotation = camera_transform.looking_at(player_transform.translation, Vec3::Y).rotation;
    
}   

fn rotate_camera_horizontal(
    mut camera: Query<(&mut Transform, &mut CameraIdentifier)>,
    mut player: Query<&Transform, (With<CameraTarget>, Without<CameraIdentifier>)>,
    mut mouse_position: EventReader<MouseMotion>,
) {
    let Ok((mut camera_transform, mut angle)) = camera.get_single_mut() else {return;};
    let Ok(player_transform) = player.get_single_mut() else {return;};
    
    for position in mouse_position.read() {
        angle.x += position.delta.x  * 0.005;
        angle.y += position.delta.y  * 0.005;



    }


    // uses sine and cosine to calculate the distance of the camera from the player
    let x = player_transform.translation.x + CAMERA_DISTANCE  * f32::cos(angle.x) ; 
    let z = player_transform.translation.z + CAMERA_DISTANCE * f32::sin(angle.x) ; 
    let y = player_transform.translation.y + CAMERA_DISTANCE * f32::sin(angle.y) ; 


    

    camera_transform.translation.x = x;
    camera_transform.translation.y = y;
    camera_transform.translation.z = z;

}

