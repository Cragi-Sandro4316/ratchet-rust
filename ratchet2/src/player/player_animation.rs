use bevy::{
    prelude::*, utils::Duration
};

use crate::{player::{Animation, CharacterController, CurrentAnimation, PlayerAnimations}, player_input::*};


pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                animation_selector,
                play_animations
            ));
    }
}


fn play_animations(
    mut animation_player: Query<&mut AnimationPlayer>,
    player: Query<(Entity, &CurrentAnimation), With<CharacterController>>,
    children: Query<&Children>,
    mut commands: Commands,
    animations: Res<PlayerAnimations>,

) {
    let Ok((player, current_animation)) = player.get_single() else {return;};

    for child in children.iter_descendants(player) {
        if let Ok(mut animation_player) = animation_player.get_mut(child) {
            match current_animation.0 {
                Animation::Idle => {
                    animation_player.play_with_transition(animations.0[4].clone_weak(), Duration::from_millis(150)).repeat();

                }
                Animation::Jump => {

                    animation_player.play_with_transition(animations.0[5].clone_weak(), Duration::from_millis(80));
                    
                    if animation_player.is_finished() {
                        commands.entity(player).remove::<Jump>();
                    }
                }
                Animation::DoubleJump => {

                    animation_player.play_with_transition(animations.0[1].clone_weak(), Duration::from_millis(80));
                    
                    if animation_player.is_finished() {
                        commands.entity(player).remove::<DoubleJump>();
                    }
                }
                Animation::Land => {
                    animation_player.play_with_transition(animations.0[6].clone_weak(), Duration::from_millis(150));
                    if animation_player.is_finished() {
                        commands.entity(player).remove::<Land>();
                        animation_player.play_with_transition(animations.0[4].clone_weak(), Duration::from_millis(150));

                    }
                }
                Animation::Walk => {
                    
                    animation_player.play_with_transition(animations.0[13].clone_weak(), Duration::from_millis(150)).set_speed(0.8).repeat();

                }
                Animation::Fall => {
                    animation_player.play_with_transition(animations.0[2].clone_weak(), Duration::from_millis(150)).repeat();

                }
                Animation::Crouch => {
                    animation_player.play_with_transition(animations.0[0].clone_weak(), Duration::from_millis(150));
                }
                Animation::SideFlipL => {
                    animation_player.play_with_transition(animations.0[8].clone_weak(), Duration::from_millis(150));
                    if animation_player.is_finished() {
                        commands.entity(player).remove::<SideflipL>();
                    }
                }
                Animation::SideFlipR => {
                    animation_player.play_with_transition(animations.0[9].clone_weak(), Duration::from_millis(150));
                    if animation_player.is_finished() {
                        commands.entity(player).remove::<SideflipR>();
                    }
                }
                Animation::Longjump => {
                    animation_player.play_with_transition(animations.0[7].clone_weak(), Duration::from_millis(150));
                    
                }
                Animation::Highjump => {
                    animation_player.play_with_transition(animations.0[3].clone_weak(), Duration::from_millis(150));
                    
                }
            }
        }
    }
}


fn animation_selector(
    mut states: Query<(
        &mut CurrentAnimation,
        Has<Idle>,
        Has<Walk>,
        Has<Land>,
        Has<Grounded>,
        Has<Falling>,
        Has<Jump>,
        Has<DoubleJump>,
        Has<Crouch>,
        Has<SideflipL>,
        Has<SideflipR>,
        Has<Longjump>,
        Has<Highjump>
    ), With<CharacterController>>
) {
    let Ok((
        mut current_animation,
        idle,
        walking,
        landing,
        grounded,
        falling,
        jumping,
        doublejumping,
        crouching,
        sideflip_l,
        sideflip_r,
        longjump,
        highjump
    )) = states.get_single_mut() else {return;};


    if grounded {
        if crouching {
            current_animation.0 = Animation::Crouch;
        }
        else if walking {
            current_animation.0 = Animation::Walk;
            
        }
        else if landing {
            current_animation.0 = Animation::Land;
        }
        else if idle {
            current_animation.0 = Animation::Idle;
        }
        
    }
    else {
        if sideflip_l {
            current_animation.0 = Animation::SideFlipL;
        }
        else if sideflip_r {
            current_animation.0 = Animation::SideFlipR;

        }
        else if longjump {
            current_animation.0 = Animation::Longjump;

        }
        else if highjump {
            current_animation.0 = Animation::Highjump;

        }
        else if jumping {
            current_animation.0 = Animation::Jump;
        }
        else if doublejumping {
            current_animation.0 = Animation::DoubleJump;
        }
        else if falling {
            current_animation.0 = Animation::Fall;
            
        }
    }

}