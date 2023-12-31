use std::f32::consts::PI;

use bevy::{prelude::*, pbr::CascadeShadowConfigBuilder};
use bevy_xpbd_3d::{components::{ComputedCollider, AsyncSceneCollider, RigidBody, Collider, Friction, CoefficientCombine, Mass}, prelude::PhysicsLayer, plugins::collision::Collisions};

use crate::player_states::{Hitbox, Damage};


pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain)
            .add_systems(Update, (
                update_crate_state,
                delete_crates
            ).chain());
    }
}

#[derive(Component)]
pub struct CrateHealth(pub f32);

#[derive(PhysicsLayer)]
pub enum Layer {
    Hittable,
    Hitbox,
    Player
}

#[derive(Component)]
pub struct CrateIdentifier;

fn spawn_terrain(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {

    // sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });


    // terrain
    commands.spawn((
        SceneBundle {
            scene: assets.load("./character_controller_demo.glb#Scene0"),
            transform: Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::PI * 0.5)),
            ..default()
        },
        
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
    ));
    


    // A cube to move around
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(2.0, 2.0, 2.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..default()
        },
    ));

    
    // Crates
    commands.spawn((
        SceneBundle {
            scene: assets.load("./bolt_crate.glb#Scene0"),
            transform: Transform::from_xyz(-2., 0.5, 1.),
            ..default()
        
        },
        Friction {
            dynamic_coefficient: 1.,
            static_coefficient: 1.,
            combine_rule: CoefficientCombine::Max
        },
        Mass(20.),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1., 1.0),
        CrateHealth(1.),
        CrateIdentifier
    ));

    commands.spawn((
        SceneBundle {
            scene: assets.load("./bolt_crate.glb#Scene0"),
            transform: Transform::from_xyz(-2., 1.5, 1.),
            ..default()
        
        },
        Friction {
            dynamic_coefficient: 1.,
            static_coefficient: 1.,
            combine_rule: CoefficientCombine::Max
        },
        Mass(20.),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1., 1.0),
        CrateHealth(1.),
        CrateIdentifier

    ));

    commands.spawn((
        SceneBundle {
            scene: assets.load("./bolt_crate.glb#Scene0"),
            transform: Transform::from_xyz(-2., 2.5, 1.),
            ..default()
        
        },
        Friction {
            dynamic_coefficient: 1.,
            static_coefficient: 1.,
            combine_rule: CoefficientCombine::Max
        },
        Mass(20.),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1., 1.0),
        CrateHealth(1.),
        CrateIdentifier
    ));

    
}


// checks if the crates have been hit
fn update_crate_state(
    mut crate_q: Query<(Entity, &mut CrateHealth), With<CrateIdentifier>>,
    hitbox_q: Query<(Entity, &Damage), (With<Hitbox>, With<Collider>)>,
    collisions: Res<Collisions>
) {
    for (crate_collider, mut crate_health) in crate_q.iter_mut() {
        for (hitbox, damage) in hitbox_q.iter() {
            if collisions.contains(hitbox, crate_collider) {
                crate_health.0 -= damage.0;
            }
        }
    }
}

fn delete_crates(
    mut commands: Commands,
    mut crate_q: Query<(Entity, &mut CrateHealth), With<CrateIdentifier>>,

) {
    for (crate_entity, crate_health) in crate_q.iter() {
        if crate_health.0 <= 0. {
            println!("deleted");
            commands.entity(crate_entity).despawn_recursive();
        }
    }
}