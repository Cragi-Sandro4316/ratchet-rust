use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};

use crate::{player_animation::PlayerAnimationPlugin, player_input::{PlayerInputPlugin, Swing}, player_movement::PlayerMovementPlugin, weapons::WeaponPlugin};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementAction>()
            .add_plugins((
                PlayerInputPlugin,
                PlayerMovementPlugin,
                PlayerAnimationPlugin,
                WeaponPlugin
            
            ))
            .add_systems(Startup, spawn_player);
    }
    
}


#[derive(Event)]
pub enum MovementAction {
    Walk(Vec2),
    Jump,
    DoubleJump,
    Gliding,
    Sideflip(Vec2),
    Longjump(Vec2),
    Highjump1,
    Highjump2,
    Swing(Vec2)
}

pub enum Animation {
    Idle,
    Jump,
    DoubleJump,
    Land,
    Walk,
    Fall,
    Crouch,
    SideFlipL,
    SideFlipR,
    Longjump,
    Highjump,
    Swing

}

// handle component for entities with character controller
#[derive(Component)]
pub struct CharacterController;

// identifier for the gun
#[derive(Component)]
pub struct Gun;

// identifier for the wrench
#[derive(Component)]
pub struct Wrench;


// handle for the camera target
#[derive(Component)]
pub struct CameraTarget;

// array containing all player animations
#[derive(Resource)]
pub struct PlayerAnimations(pub Vec<Handle<AnimationClip>>);

// array containing all wrench animations
#[derive(Resource)]
pub struct WrenchAnimations(pub Vec<Handle<AnimationClip>>);

// the animation currently being played
#[derive(Component)]
pub struct CurrentAnimation(pub Animation);

// the direction the player is walking 
#[derive(Component)]
pub struct PlayerDirection(pub Vec2);

// [MOVEMENT COMPONENTS]

// the player acceleration
#[derive(Component)]
pub struct MovementAcceleration(pub Scalar);

// how much the player slows down
#[derive(Component)]
pub struct MovementDampingFactor(pub Scalar);

// the force at the start of the jump
#[derive(Component)]
pub struct JumpImpulse(pub Scalar);

// The force at the start of a double jump.
#[derive(Component)]
pub struct DoubleJumpImpulse(pub Scalar);

// counts the jumps and the last time jumped
#[derive(Component)]
pub struct JumpCounter {
    pub counter: Scalar,
    pub jump_time: Scalar
}

// determines what's the maximum angle the player can walk on before slipping
#[derive(Component)]
pub struct MaxSlopeAngle(pub Scalar);

// the height value of the player the last time it was grounded
#[derive(Component)]
pub struct GroundedHeight(pub f32);

// [BUNDLES]
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}
impl CharacterControllerBundle {
    fn new(collider: Collider) -> Self {
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
                Direction3d::NEG_Y,
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



#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    double_jump_impulse: DoubleJumpImpulse,
    jump_counter: JumpCounter,
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
            jump_counter: JumpCounter {
                counter: 0.,
                jump_time: -1.            
            },
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, 7.0,  PI * 0.45)
    }
}








fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    commands.insert_resource(PlayerAnimations(vec![
        assets.load("ratchet2.glb#Animation0"),
        assets.load("ratchet2.glb#Animation1"),
        assets.load("ratchet2.glb#Animation2"),
        assets.load("ratchet2.glb#Animation3"),
        assets.load("ratchet2.glb#Animation4"),
        assets.load("ratchet2.glb#Animation5"),
        assets.load("ratchet2.glb#Animation6"),
        assets.load("ratchet2.glb#Animation7"),
        assets.load("ratchet2.glb#Animation8"),
        assets.load("ratchet2.glb#Animation9"),
        assets.load("ratchet2.glb#Animation10"),
        assets.load("ratchet2.glb#Animation11"),
        assets.load("ratchet2.glb#Animation12"),
        assets.load("ratchet2.glb#Animation13"),
        // other animations here
    ]));

    // Player
    let player = commands.spawn((
        SceneBundle {
            scene: assets.load("ratchet2.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 5.5, 0.0),
            ..default()
        },
        
        CharacterControllerBundle::new(Collider::capsule(0.4, 0.4)).with_movement(
            65.0,
            0.92,
            11.2,
            9.,
            (45.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(3.0),
        CameraTarget,
        CurrentAnimation(Animation::Idle),
        PlayerDirection(Vec2::ZERO),
        Swing {
            swing_time: 0.,
            swing_number: 0
        },
        GroundedHeight(0.)
        
    )).id();



    commands.insert_resource(WrenchAnimations(vec![
        assets.load("wrench.glb#Animation0"),
        assets.load("wrench.glb#Animation1"),
        assets.load("wrench.glb#Animation2"),
        assets.load("wrench.glb#Animation3"),
        assets.load("wrench.glb#Animation4"),
        assets.load("wrench.glb#Animation5"),
        assets.load("wrench.glb#Animation6"),
        assets.load("wrench.glb#Animation7"),
        assets.load("wrench.glb#Animation8"),
        assets.load("wrench.glb#Animation9"),
        assets.load("wrench.glb#Animation10"),
        assets.load("wrench.glb#Animation11"),
        assets.load("wrench.glb#Animation12"),
        assets.load("wrench.glb#Animation13"),
        // other animations here
    ]));

    let wrench = commands.spawn((
        SceneBundle {
            scene: assets.load("wrench.glb#Scene0"),
            visibility: Visibility::Visible,
            ..default()
        },
        Wrench
    )).id();

    commands.entity(player).push_children(&[wrench]);


    let gun = commands.spawn((
        SceneBundle {
            scene: assets.load("blastatore.glb#Scene0"),
            transform: Transform::from_xyz(0.155, 0.06, -0.33).with_scale(Vec3::splat(0.12)),
            visibility: Visibility::Hidden,
            ..default()
        },
        Gun
    )).id();

    commands.entity(player).push_children(&[gun]);
}

