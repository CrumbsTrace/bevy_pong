pub struct MovementPlugin;

use crate::components::{Ball, CollisionEvent, MovementKeys, PongSystemSet, Velocity};
use crate::constants::*;
use crate::PlayState;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

use super::world_builder::BallSpeed;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                move_paddles,
                move_ball.in_set(OnUpdate(PlayState::Playing)),
                handle_bounce
                    .before(move_ball)
                    .run_if(on_event::<CollisionEvent>()),
            )
                .in_set(PongSystemSet::GameLogic),
        )
        .add_event::<CollisionEvent>();
    }
}

fn move_paddles(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &MovementKeys)>,
) {
    for (mut transform, movement_keys) in query.iter_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(movement_keys.up) {
            direction += 1.0;
        }
        if keyboard_input.pressed(movement_keys.down) {
            direction -= 1.0;
        }

        if direction == 0.0 {
            continue;
        }

        let new_position =
            transform.translation.y + direction * PADDLE_SPEED * time.delta().as_secs_f32();

        //Clamp paddle position
        let top_bound = TOP_WALL - PADDLE_SIZE.y / 2.0 - WALL_THICKNESS / 2.0;
        let bottom_bound = BOTTOM_WALL + PADDLE_SIZE.y / 2.0 + WALL_THICKNESS / 2.0;

        transform.translation.y = new_position.clamp(bottom_bound, top_bound);
    }
}

fn move_ball(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut ball_speed: ResMut<BallSpeed>,
) {
    for (mut transform, mut velocity) in ball_query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        // update velocity based on the current ball speed
        let normalized_velocity = velocity.normalize();
        velocity.x = ball_speed.0 * normalized_velocity.x;
        velocity.y = ball_speed.0 * normalized_velocity.y;
    }

    ball_speed.0 += BALL_ACCELERATION * time.delta_seconds();
}

fn handle_bounce(
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut ball_velocity = ball_query.single_mut();

    for collision in collision_events.iter() {
        let mut reflect_x = false;
        let mut reflect_y = false;

        match collision.collision {
            Collision::Left => reflect_x = ball_velocity.x > 0.0,
            Collision::Right => reflect_x = ball_velocity.x < 0.0,
            Collision::Top => reflect_y = ball_velocity.y < 0.0,
            Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            Collision::Inside => { /* do nothing */ }
        }

        if reflect_x {
            ball_velocity.x = -ball_velocity.x;
        }

        if reflect_y {
            ball_velocity.y = -ball_velocity.y;
        }
    }
}
