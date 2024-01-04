use bevy::prelude::*;

use crate::{player::{CurrentAnimation, CharacterController, PlayerAnimations, Grounded}, player_states::*};

pub struct PlayerAnimationsPlugin; 

impl Plugin for PlayerAnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            (
                animation_selector,
                play_animations
            ).chain()
        );
    }
}


fn play_animations(
    mut anim_player_q: Query<(&mut AnimationPlayer, Entity)>,
    animation_event: Query<&mut CurrentAnimation, With<CharacterController>> ,
    animations: Res<PlayerAnimations>,

    player_q: Query<Entity, With<CharacterController>>,
    mut commands: Commands,
    parent_query: Query<&Parent>,
) {
    let Ok(animation) = animation_event.get_single() else {return;};
    let Ok(player_entity) = player_q.get_single() else {return;};

    
    for (mut anim_player, anim_player_entity) in anim_player_q.iter_mut() {
        for parent in parent_query.iter_ancestors(anim_player_entity) {
            for grandparent in parent_query.iter_ancestors(parent) {
                if commands.entity(grandparent).id() == commands.entity(player_entity).id() {
                    match animation.0.as_str() {
                                "IDLE" => {
                                    commands.entity(player_entity).remove::<HighJump>();
                                    commands.entity(player_entity).remove::<DoubleJump>();
                                    anim_player.play(animations.0[3].clone_weak()).repeat();
                    
                                }
                                "JUMP" => {
                                    anim_player.play(animations.0[4].clone_weak());
                                    if anim_player.is_finished() {
                                        commands.entity(player_entity).remove::<Jump>();
                                    }
                                }
                                "DOUBLEJUMP" => {
                                    anim_player.play(animations.0[1].clone_weak());
                                    if anim_player.is_finished() {
                                        commands.entity(player_entity).remove::<DoubleJump>();
                                    }
                                }
                                "WALK" => {
                                    commands.entity(player_entity).remove::<HighJump>();
                                    commands.entity(player_entity).remove::<DoubleJump>();
                                    anim_player.play(animations.0[9].clone_weak()).repeat();
                                    
                                }
                                "SWING1" => {
                                    anim_player.play(animations.0[8].clone_weak());
                                }
                                "GLIDE" => {
                                    anim_player.play(animations.0[2].clone_weak()).repeat();
                                }
                                "CROUCH" => {
                                    anim_player.play(animations.0[0].clone_weak());
                                }
                                "HIGHJUMP" => {
                                    anim_player.play(animations.0[6].clone_weak());
                                    if anim_player.is_finished() {
                                        commands.entity(player_entity).remove::<HighJump>();

                                    }
                                }
                                "LONGJUMP" => {
                                    anim_player.play(animations.0[5].clone_weak());
                                    
                                    if anim_player.is_finished() {
                                        commands.entity(player_entity).remove::<LongJump>();
                                    }
                                }
                                _ => {}
                            }
                }
            }
        }

    }
        
}





pub fn animation_selector(
    mut player_query: Query<(
        Has<Idle>,
        Has<Walking>,
        Has<Jump>,
        Has<DoubleJump>,
        Has<Swing1>,
        Has<Falling>,
        Has<Grounded>,
        Has<Gliding>,
        Has<Crouch>,
        Has<HighJump>,
        Has<LongJump>,
        &mut CurrentAnimation
    ), With<CharacterController>>
) {
    let Ok((
        is_idle,
        is_walking,
        is_jumping,
        is_double_jumping,
        is_swinging,
        is_falling,
        is_grounded,
        is_gliding,
        is_crouching,
        is_high_jumping,
        is_long_jumping,
        mut current_animation
    )) = player_query.get_single_mut() else {return;};


    if !is_grounded {
        if is_long_jumping {
            current_animation.0 = "LONGJUMP".to_owned();
        }
        else if is_high_jumping {
            current_animation.0 = "HIGHJUMP".to_owned();
        }
        else if is_jumping {
            current_animation.0 = "JUMP".to_owned();

        }
        else if is_double_jumping {
            current_animation.0 = "DOUBLEJUMP".to_owned();
        } 
        else if is_falling {
            if is_gliding {
                current_animation.0 = "GLIDE".to_owned();
            }
            else {
                current_animation.0 = "JUMP".to_owned();
            }
        }

    } else {
        if is_swinging {
            current_animation.0 = "SWING1".to_owned();
        }
        else {
             if is_crouching {
                current_animation.0 = "CROUCH".to_owned();
            }
            else if is_walking {
                current_animation.0 = "WALK".to_owned();
            }
            else if is_idle {
                current_animation.0 = "IDLE".to_owned();
            }
            
        }
    }

    // if is_jumping {
    //     current_animation.0 = "JUMP".to_owned();
    // }
    // else if is_double_jumping {
    //     current_animation.0 = "DOUBLEJUMP".to_owned();
    // }
    // else if is_grounded {
    //     if is_swinging {
    //         current_animation.0 = "SWING1".to_owned();
    //     }
    //     else {
    //         if is_walking {
    //             current_animation.0 = "WALK".to_owned();
    //         }
    //         else if is_idle {
    //             current_animation.0 = "IDLE".to_owned();
    //         }
    //     }
    // }


}