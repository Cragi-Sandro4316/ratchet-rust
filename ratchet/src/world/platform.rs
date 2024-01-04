use std::f32::consts::PI;

use bevy::{prelude::*, pbr::CascadeShadowConfigBuilder};
use bevy_xpbd_3d::{
    components::{
        ComputedCollider, 
        AsyncSceneCollider, 
        RigidBody, Collider, 
        Friction, 
        CoefficientCombine, 
        Mass
    }, 
    prelude::PhysicsLayer, 
    plugins::collision::Collisions
};

use rand::Rng;

use crate::{player_states::{Hitbox, Damage}, player::{CharacterController, Bolts}, /*player::CharacterController*/};


pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain)
            .add_systems(Update, (
                update_crate_state,
                delete_crates,
                crate_animator
            ).chain());
    }
}

#[derive(Component)]
pub struct CrateHealth(pub f32);

#[derive(PhysicsLayer)]
pub enum Layer {
    Hittable,
    //Hitbox,
    Player
}

// array containing all player animations
#[derive(Resource)]
pub struct CrateAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Component)]
pub struct CrateIdentifier;

#[derive(Component)]
pub struct Dead;

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
            scene: assets.load("./character_controller2.glb#Scene0"),
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


    commands.insert_resource(CrateAnimations(vec![
        assets.load("bolt_crate.glb#Animation0"),

        // other animations here
    ]));
    
    // Crates
    commands.spawn((
        SceneBundle {
            scene: assets.load("./bolt_crate.glb#Scene0"),
            transform: Transform::from_xyz(-2., 0.6, 1.),
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
        CrateIdentifier,
    ));

    commands.spawn((
        SceneBundle {
            scene: assets.load("./bolt_crate.glb#Scene0"),
            transform: Transform::from_xyz(-2., 1.8, 1.),
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
        CrateIdentifier,

    ));

    commands.spawn((
        SceneBundle {
            scene: assets.load("./bolt_crate.glb#Scene0"),
            transform: Transform::from_xyz(-2., 3., 1.),
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
        CrateIdentifier,

    ));

    
}


// checks if the crates have been hit
fn update_crate_state(
    mut crate_q: Query<(Entity, &mut CrateHealth), With<CrateIdentifier>>,
    hitbox_q: Query<(Entity, &Damage), (With<Hitbox>, With<Collider>)>,
    collisions: Res<Collisions>,
    //time: Res<Time>
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
    crate_q: Query<(Entity, &mut CrateHealth), With<CrateIdentifier>>,

) {
    for (crate_entity, crate_health) in crate_q.iter() {
        if crate_health.0 <= 0. {
            commands.entity(crate_entity).insert(Dead);
        }
    }
}

fn crate_animator(
    mut anim_player_q: Query<(&mut AnimationPlayer, Entity)>,
    animations: Res<CrateAnimations>,
    crate_q: Query<Entity, (With<CrateIdentifier>, With<Dead>)>,
    mut commands: Commands,
    parent_query: Query<&Parent>,
    mut player_bolts: Query<&mut Bolts, With<CharacterController>>
) {
    let Ok(mut bolts) = player_bolts.get_single_mut() else {return;};
    
    for (mut anim_player, anim_player_entity) in &mut anim_player_q.iter_mut() {
        for parent in parent_query.iter_ancestors(anim_player_entity) {
            for grandparent in parent_query.iter_ancestors(parent) {
                for crates in crate_q.iter() {
                    if commands.entity(grandparent).id() == commands.entity(crates).id() {
                        anim_player.play(animations.0[0].clone_weak());

                        if anim_player.is_finished() {
                            commands.entity(crates).despawn_recursive();
                            bolts.0 += rand::thread_rng().gen_range(60..200);
                            println!("bolts: {}", bolts.0);
                        }
                    }
                }
            }
        }
    }
}