use bevy::prelude::*;
use bevy_xpbd_3d::{
    components::{
        ComputedCollider, 
        AsyncSceneCollider, 
        RigidBody, Collider, 
        Friction, 
        CoefficientCombine, 
        Mass
    }, 
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


    commands
        .spawn(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform {
                translation: Vec3::new(75., 635., 0.),
                ..default()
            },
            point_light: PointLight {
                intensity: 17000000.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::WHITE,
                range: 10000.0,
                shadows_enabled: false,
                ..default()
            },
            ..default()
        });


    // terrain
    commands.spawn((
        SceneBundle {
            scene: assets.load("./metropolis_collisions.glb#Scene0"),
            transform: Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::PI * 0.5)),
            visibility: Visibility::Hidden,
            ..default()
        },
        
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
    ));
    
    commands.spawn((
        SceneBundle {
            scene: assets.load("./metropolis.glb#Scene0"),
            transform: Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::PI * 0.5)),
            ..default()
        },
        
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



    commands.spawn(
        SceneBundle {
            scene: assets.load("./blastatore.glb#Scene0"),
            transform: Transform::from_xyz(3.0, 5.0, 3.0).with_scale(Vec3::splat(1.)),
            ..default()
        }
    );
    
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