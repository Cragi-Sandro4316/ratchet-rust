use bevy::prelude::*;

use crate::{player::{Animations, CurrentAnimation, CharacterController, JumpCounter}, player_states::*};

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
    mut anim_player_q: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    animation_event: Query<&mut CurrentAnimation, With<CharacterController>> ,
    

) {
    let Ok(animation) = animation_event.get_single() else {return;};

    for mut anim_player in &mut anim_player_q {
        match animation.0.as_str() {
            "IDLE" => {
                anim_player.play(animations.0[1].clone_weak()).repeat();
            }
            "JUMP" => {
                anim_player.play(animations.0[2].clone_weak());
            }
            "DOUBLEJUMP" => {
                anim_player.play(animations.0[0].clone_weak());
            }
            "WALK" => {
                anim_player.play(animations.0[4].clone_weak()).repeat();
            }
            "SWING1" => {
                anim_player.play(animations.0[3].clone_weak());
            }
            
            _ => {}
        }
    }
        
}


pub fn animation_selector(
    mut player_query: Query<(
        Has<Idle>,
        Has<Walking>,
        Has<Jump>,
        Has<Swing1>,
        &JumpCounter,
        &mut CurrentAnimation
    ), With<CharacterController>>
) {
    let Ok((
        is_idle,
        is_walking,
        is_jumping,
        is_swinging,
        jump_counter,
        mut current_animation
    )) = player_query.get_single_mut() else {return;};

    if is_jumping {
        if jump_counter.counter < 2. {
            current_animation.0 = "JUMP".to_owned();
        }
        else {
            current_animation.0 = "DOUBLEJUMP".to_owned();
        }
    }
    else {
        if is_swinging {
            current_animation.0 = "SWING1".to_owned();
        }
        else {
            if is_walking {
                current_animation.0 = "WALK".to_owned();
            }
            else if is_idle {
                current_animation.0 = "IDLE".to_owned();
            }
        }
    }

}