use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_editor_pls::EditorPlugin;
use bevy_xpbd_3d::plugins::PhysicsPlugins;

#[path = "./camera/camera.rs"]
mod camera;
use crate::camera::CameraPlugin;

#[path = "./level/level.rs"]
mod level;
use crate::level::LevelPlugin;

#[path = "./player/player_setup.rs"]
mod player;
use crate::player::PlayerPlugin;

#[path = "./player/player_input.rs"]
mod player_input;

#[path = "./player/player_movement.rs"]
mod player_movement;

#[path = "./player/player_animation.rs"]
mod player_animation;

#[path = "./player/weapons.rs"]
mod weapons;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EditorPlugin::default(),
            CameraPlugin,
            LevelPlugin,
            PlayerPlugin,
            PhysicsPlugins::default(),
        
        ))
        .add_systems(Startup, windows_settings)
        .run();
}



fn windows_settings(
    mut window: Query<&mut Window, With<PrimaryWindow>>,

) {
    let window = &mut window.single_mut();

    window.title = "Ratchet & Clank: Computer coming".to_string();

    // window.present_mode = PresentMode::AutoNoVsync;

}