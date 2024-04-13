use bevy::prelude::*;

use crate::player::CameraTarget;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera_position);
    }
}

#[derive(Component)]
pub struct CameraIdentifier {
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
        CameraIdentifier {
            x: 0.,
            y: 0.
        },
    ));
}

const CAMERA_DISTANCE: f32 = 6.5;

fn update_camera_position(
    mut camera: Query<(&mut Transform, &mut CameraIdentifier)>,
    target: Query<&Transform, (With<CameraTarget>, Without<CameraIdentifier>)>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    time: Res<Time>
) {
    let Ok((mut camera_transform, mut camera_angle)) = camera.get_single_mut() else {return;};
    let Ok(target_transform) = target.get_single() else {return;};

    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::RightStickX
        };
    
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::RightStickY
        };
    
        // if the analog stick is pointing to any direction it adds the value to the camera angle
        if let(Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            camera_angle.x += x * time.delta_seconds() * 2.;
            

            if camera_angle.y + y * time.delta_seconds() * 2. < 1.41372 
            && camera_angle.y + y * time.delta_seconds() * 2. > -1.41372 {
                camera_angle.y += y * time.delta_seconds() * 2.;

            }

        }


    }


    // if the camera has done a full circle around the player it resets the angle 
    if camera_angle.x > 6.28319 || camera_angle.x < -6.28319 {
        camera_angle.x = 0.;
    }


    // calculates the camera position through trigonometry and adds the player position to it
    let x = target_transform.translation.x + CAMERA_DISTANCE * f32::cos(camera_angle.x); 
    let z = target_transform.translation.z + CAMERA_DISTANCE * f32::sin(camera_angle.x);
    let y = target_transform.translation.y + CAMERA_DISTANCE * f32::sin(camera_angle.y); 

    camera_transform.translation.x = x;
    camera_transform.translation.y = y;
    camera_transform.translation.z = z;


    *camera_transform = camera_transform.looking_at(target_transform.translation, Vec3::Y);

}
