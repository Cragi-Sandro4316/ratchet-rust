use bevy::prelude::*;
use bevy_editor_pls::*;
use bevy_xpbd_3d::plugins::PhysicsPlugins;
use platform::PlatformPlugin;

#[path ="./player/player.rs"]
mod player;

#[path ="./player/player_animations.rs"]
mod player_animations;

#[path ="./player/player_movement.rs"]
mod player_movement;

#[path ="./player/player_states.rs"]
mod player_states;

#[path ="./camera/camera.rs"]
mod camera;

#[path ="./world/platform.rs"]
mod platform;


use crate::player::CharacterControllerPlugin;
use crate::camera::CameraPlugin_;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CharacterControllerPlugin,
            CameraPlugin_,
            PhysicsPlugins::default(),
            PlatformPlugin,
            EditorPlugin::default()
        ))
        .run();
}


