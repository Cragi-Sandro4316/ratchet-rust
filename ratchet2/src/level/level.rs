use bevy::{audio::{PlaybackMode, Volume}, pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_xpbd_3d::{components::RigidBody, plugins::collision::{AsyncSceneCollider, ComputedCollider}};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_terrain, 
                level_music
            ));
    }
}

#[derive(Component)]
pub struct LevelCollider;

fn spawn_terrain(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    // [LIGHTS]
    commands.insert_resource(AmbientLight {
        color: Color::Rgba { red: (0.91), green: (1.), blue: (0.93), alpha: (1.) },
        brightness: 220.,
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            //rotation: Quat::from_rotation_x(-3.14 / 2.6),
            rotation: Quat::from_rotation_x(-3.14 / 2.),

            ..default()
        },
        
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 350.0,
            ..default()
        }
        .into(),
        ..default()
    });

   


    // [TERRAIN COLLISION]
    commands.spawn((
        SceneBundle {
            scene: assets.load("./insomniac_museum_collisions.glb#Scene0"),
            //scene: assets.load("./metropolis_collisions.glb#Scene0"),
            //scene: assets.load("./veldin_collisions.glb#Scene0"),
            
            transform: Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::PI * 0.5)),
            visibility: Visibility::Hidden,
            ..default()
        },
        LevelCollider,
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
    ));
    
    // [TERRAIN ]
    commands.spawn((
        SceneBundle {
            scene: assets.load("./insomniac_museum.glb#Scene0"),
            //scene: assets.load("./metropolis.glb#Scene0"),
            //scene: assets.load("./veldin.glb#Scene0"),

            transform: Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::PI * 0.5)),
            ..default()
        },
        
        RigidBody::Static,
    ));

}


fn level_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("metropolis.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.05),
                ..default()
            },
        },
        
    ));

}