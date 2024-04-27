use bevy::{math::vec3, prelude::*};
use bevy_xpbd_3d::plugins::collision::{Collider, Collisions};

use crate::{level::LevelCollider, player::{CharacterController, Gun, MovementAction, Wrench}, player_input::{Grounded, Swing}};

pub struct WeaponPlugin;

const BULLET_SPEED: f32 = 25.; 

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub shoot_time: f32
}

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            shoot, 
            update_bullet_pos,
            swing
        ));
    }
}

fn shoot(
    player: Query<&Transform, With<CharacterController>>,
    mut gun: Query<&mut Visibility, (With<Gun>, Without<Wrench>)>,
    mut wrench: Query<&mut Visibility, With<Wrench>>,
    shots: Query<&Bullet>,
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    let Ok(mut gun_visibility) = gun.get_single_mut() else {return;};
    let Ok(mut wrench_visibility) = wrench.get_single_mut() else {return;};

    let Ok(transform) = player.get_single() else {return;};


    for gamepad in gamepads.iter() {
        let fire = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::East
        };


        let mut last_shot_time = 0.;

        for shot in shots.iter() {
            if shot.shoot_time > last_shot_time {
                last_shot_time = shot.shoot_time;
            }
        }


        if buttons.pressed(fire) && last_shot_time + 0.1 < time.elapsed_seconds()  {
            let direction = vec3(
                transform.forward().x,
                transform.forward().y,
                transform.forward().z, 
            );

            *wrench_visibility = Visibility::Hidden; 

            *gun_visibility = Visibility::Visible;

            let gun_position = Vec3::new(
                transform.translation.x + 0.155,
                transform.translation.y + 0.06,
                transform.translation.z + 0.23
            ) + direction * 1.5;

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.1)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::YELLOW,
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        gun_position.x, 
                        gun_position.y, 
                        gun_position.z
                    ),
                    ..default()
                },
                Collider::sphere(0.6),
                Bullet {
                    direction: direction,
                    shoot_time: time.elapsed_seconds()
                }
                
            ));
        } 
    }


}


fn update_bullet_pos(
    mut shots: Query<(Entity, &Bullet, &mut Transform)>,
    
    mut commands: Commands,
    time: Res<Time>,
    collisions: Res<Collisions>,
    level_collider: Query<Entity, With<LevelCollider>>,
    children: Query<&Children>,

) {
    let Ok(level) = level_collider.get_single() else {return;};

    for (bullet_entity, shot, mut transform) in shots.iter_mut() {
        transform.translation += shot.direction * BULLET_SPEED * time.delta_seconds(); 

        for child in children.iter_descendants(level) {
            if collisions.contains(child, bullet_entity) {
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

fn swing(
    mut player: Query<(&mut Swing, Has<Grounded>, &Transform), With<CharacterController>>,
    mut gun: Query<&mut Visibility, (With<Gun>, Without<Wrench>)>,
    mut wrench: Query<&mut Visibility, With<Wrench>>,
    mut movement_event: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    time: Res<Time>,
) {
    let Ok((mut swing, grounded, transfom)) = player.get_single_mut() else {return;};
    let Ok(mut gun_visibility) = gun.get_single_mut() else {return;};
    let Ok(mut wrench_visibility) = wrench.get_single_mut() else {return;};


    for gamepad in gamepads.iter() {
        let swing_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::West
        };




        if grounded {
            if buttons.just_pressed(swing_button) 
            && swing.swing_number < 3
            && swing.swing_time + 0.25 < time.elapsed_seconds() {

                *gun_visibility = Visibility::Hidden;
                *wrench_visibility = Visibility::Visible; 

                swing.swing_number += 1;
                swing.swing_time = time.elapsed_seconds();
                
                let direction = Vec2::new(
                    transfom.forward().x, 
                    transfom.forward().z, 
                );

                movement_event.send(MovementAction::Swing(direction));

            }
        
        }

        if swing.swing_time + 0.5 < time.elapsed_seconds() {
            swing.swing_number = 0;

        }


    }    


}