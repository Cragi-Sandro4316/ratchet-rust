use bevy::prelude::*;
use bevy_xpbd_3d::components::{ComputedCollider, AsyncSceneCollider, RigidBody, Collider};


pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain);
    }
}



fn spawn_terrain(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    
    // Environment (see `async_colliders` example for creating colliders from scenes)
    commands.spawn((
        SceneBundle {
            scene: assets.load("character_controller_demo.glb#Scene0"),
            transform: Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::PI * 0.5)),
            ..default()
        },
        
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
    ));


    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 6000.0,
            range: 50.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 0.0),
        ..default()
    });

    // A cube to move around
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(3.0, 2.0, 3.0),
            ..default()
        },
    ));


    
}