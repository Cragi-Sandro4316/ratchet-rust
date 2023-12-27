use bevy::{prelude::*, input::mouse::MouseMotion, window::{PrimaryWindow, CursorGrabMode}};
use bevy_third_person_camera::*;

use crate::player::CameraTarget;

pub struct CameraPlugin_;

impl Plugin for CameraPlugin_ {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                update_camera_pos, 
                orbit_camera, 
                hide_cursor
                
            ));
    }
}


#[derive(Component)]
pub struct CameraIdentifier{
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub struct CursorVisible(bool);

#[derive(Component)]
pub struct MovementHelper;

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
        },
        CursorVisible(false),
    ));

    commands.spawn((MovementHelper, Transform::default()));
}

const CAMERA_DISTANCE: f32 = 7.;

fn update_camera_pos(
    mut camera: Query<&mut Transform, With<CameraIdentifier>>,
    mut player: Query<&Transform, (With<CameraTarget>, Without<CameraIdentifier>)>,


) {
    let Ok(mut camera_transform) = camera.get_single_mut() else {return;};
    let Ok(player_transform) = player.get_single_mut() else {return;};
    camera_transform.rotation = camera_transform.looking_at(player_transform.translation, Vec3::Y).rotation;
    
}   

fn orbit_camera(
    mut camera: Query<(&mut Transform, &mut CameraIdentifier)>,
    player: Query<&Transform, (With<CameraTarget>, Without<CameraIdentifier>)>,
    mut movement_helper: Query<&mut Transform, (With<MovementHelper>, Without<CameraTarget>, Without<CameraIdentifier>)>,
    mut mouse_position: EventReader<MouseMotion>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,

) {
    let Ok((mut camera_transform, mut angle)) = camera.get_single_mut() else {return;};
    let Ok(player_transform) = player.get_single() else {return;};
    let Ok(mut movement_helper) = movement_helper.get_single_mut() else {return;};
    
    // mouse
    for position in mouse_position.read() {
        angle.x += position.delta.x  * 0.005;
        angle.y += position.delta.y  * 0.005;

    }

    // gamepad
    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::RightStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::RightStickY,
        };

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            angle.x += x  * 0.02;

            angle.y += y  * 0.02;
            
            
        }
    }


    // uses sine and cosine to calculate the distance of the camera from the player
    let x = player_transform.translation.x + CAMERA_DISTANCE  * f32::cos(angle.x) ; 
    let z = player_transform.translation.z + CAMERA_DISTANCE * f32::sin(angle.x) ; 
    let y = player_transform.translation.y + CAMERA_DISTANCE * f32::sin(angle.y) ; 

    movement_helper.translation.x = x;
    movement_helper.translation.z = z;
    movement_helper.look_at(player_transform.translation, Vec3::Y);

    camera_transform.translation.x = x;
    camera_transform.translation.y = y;
    camera_transform.translation.z = z;

}

fn hide_cursor(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_visible: Query<&mut CursorVisible>,
    input: Res<Input<KeyCode>>
) {
    let window = &mut window.single_mut();
    let Ok(mut cursor_visible) = cursor_visible.get_single_mut() else {return;};
    
    
    if input.any_just_pressed([KeyCode::P]) {
        cursor_visible.0 = !cursor_visible.0;
    }

    if !cursor_visible.0 {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    } 
    else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }

}