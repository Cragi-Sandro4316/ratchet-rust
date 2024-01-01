use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};

use crate::{ player_animations::PlayerAnimationsPlugin, player_movement::PlayerMovementPlugin, player_states::PlayerStatePlugin};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
        .add_plugins((
            PlayerAnimationsPlugin,
            PlayerMovementPlugin,
            PlayerStatePlugin
        ))
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (
                apply_deferred,
                
            )
                .chain(),
        );
    }
}



/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
    Jump,
    DoubleJump,
    Swing1(Vector2),
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(pub Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(pub Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(pub Scalar);

/// The strength of a double jump.
#[derive(Component)]
pub struct DoubleJumpImpulse(pub Scalar);

#[derive(Component)]
pub struct JumpCounter {
    pub counter: Scalar,
    pub jump_time: Scalar
}

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(pub Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    double_jump_impulse: DoubleJumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        double_jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            double_jump_impulse: DoubleJumpImpulse(double_jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, 7.0,  PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Vector::NEG_Y,
            )
            .with_max_time_of_impact(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        double_jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, double_jump_impulse, max_slope_angle);
        self
    }
}


// array containing all player animations
#[derive(Resource)]
pub struct PlayerAnimations(pub Vec<Handle<AnimationClip>>);

// the animation being currently played
#[derive(Component)]
pub struct CurrentAnimation(pub String);

#[derive(Component)]
pub struct Bolts(pub i32);

#[derive(Component)]
pub struct CameraTarget;

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    commands.insert_resource(PlayerAnimations(vec![
        assets.load("og_ratchet.glb#Animation0"),
        assets.load("og_ratchet.glb#Animation1"),
        assets.load("og_ratchet.glb#Animation2"),
        assets.load("og_ratchet.glb#Animation3"),
        assets.load("og_ratchet.glb#Animation4"),
        assets.load("og_ratchet.glb#Animation5"),
        assets.load("og_ratchet.glb#Animation6"),
        assets.load("og_ratchet.glb#Animation7"),


        // other animations here
    ]));

    // Player
    commands.spawn((
        SceneBundle {
            scene: assets.load("og_ratchet.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(1.2, 0.4)).with_movement(
            65.0,
            0.92,
            11.2,
            9.,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(3.0),
        CameraTarget,
        JumpCounter {
            counter: 0.,
            jump_time: -1.
        },
        CurrentAnimation("IDLE".to_owned()),
        Bolts(0)        
    ));
}








