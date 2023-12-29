use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};

use crate::camera::MovementHelper;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (
                update_grounded,
                keyboard_input,
                gamepad_input,
                apply_deferred,
                movement,
                apply_movement_damping,
                
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
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The strength of a double jump.
#[derive(Component)]
pub struct DoubleJumpImpulse(Scalar);

#[derive(Component)]
pub struct JumpCounter {
    pub counter: Scalar,
    pub jump_time: Scalar
}

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

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

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
    camera: Query<&Transform, With<MovementHelper>>,


) {
    let Ok(camera_transform) = camera.get_single() else {return;};

    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

    let vertical = (up as i8 - down as i8) as f32 * Vec2::new(camera_transform.forward().x, -camera_transform.forward().z);
    let horizontal =  (right as i8 - left as i8) as f32 * Vec2::new(camera_transform.right().x, -camera_transform.right().z);

    let direction = horizontal + vertical;
   
    if direction != Vector2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction.normalize()));
    }

    if keyboard_input.any_just_pressed([KeyCode::Space]) {
        movement_event_writer.send(MovementAction::Jump);
        
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    camera: Query<&Transform, With<MovementHelper>>,
    

) {
    let Ok(camera_transform) = camera.get_single() else {return;};

    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            let vertical = y * Vec2::new(camera_transform.forward().x, -camera_transform.forward().z);
            let horizontal =  x * Vec2::new(camera_transform.right().x, -camera_transform.right().z);


            let direction = horizontal + vertical;
            
            movement_event_writer.send(MovementAction::Move(direction));
        }

        let jump_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };

        if buttons.just_pressed(jump_button) {
            movement_event_writer.send(MovementAction::Jump);
        }
        
        
        
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
    mut jump_counter: Query<&mut JumpCounter>,
    time: Res<Time>
) {
    let Ok(mut jump_counter) = jump_counter.get_single_mut() else {return;};
    
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
            // waits half a second after a jump before resetting the counter 
            // to avoid the counter being reset on the first jump frame
            if time.elapsed_seconds() > jump_counter.jump_time + 0.5 {
                jump_counter.counter = 0.;
            }

        } else {
            commands.entity(entity).remove::<Grounded>();
            
        }
    }
}



/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &DoubleJumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
        &mut JumpCounter
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();



    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, double_jump_impulse, mut linear_velocity, is_grounded, mut jump_counter) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {

                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.z -= direction.y * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    if is_grounded {
                        jump_counter.jump_time = time.elapsed_seconds();
                        linear_velocity.y = jump_impulse.0;
                        jump_counter.counter += 1.;
                    } else if jump_counter.counter < 2. && jump_counter.counter > 0. && time.elapsed_seconds() < jump_counter.jump_time + 0.85 {
                        linear_velocity.y = double_jump_impulse.0;
                        jump_counter.counter += 1.;
                        
                    }
                }
            }
        }
    }
}

/// Slows down movement in the XZ plane.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}


#[derive(Component)]
pub struct CameraTarget;

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Player
    commands.spawn((
        SceneBundle {
            scene: assets.load("ratchet.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(1.2, 0.4)).with_movement(
            40.0,
            0.92,
            9.0,
            7.5,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
        CameraTarget,
        JumpCounter {
            counter: 0.,
            jump_time: -1.
        }
    ));
}





