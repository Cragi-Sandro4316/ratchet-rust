use bevy::prelude::*;
use bevy_editor_pls::*;
use bevy_xpbd_3d::plugins::PhysicsPlugins;
use platform::PlatformPlugin;

#[path ="./player/player.rs"]
mod player;

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


