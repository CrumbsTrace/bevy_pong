pub struct MovementPlugin;

use crate::components::{Ball, CollisionEvent, MovementKeys, Velocity};
use crate::constants::*;
use crate::PlayState;
use bevy_prototype_debug_lines::DebugLines;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((move_paddles, move_ball, handle_bounce.before(move_ball)));
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
    mut ball_query: Query<(&mut Transform, &Velocity)>,
    game_state: Res<PlayState>,
    mut lines: ResMut<DebugLines>,
) {
    if game_state.is_paused() {
        return;
    }

    for (mut transform, velocity) in ball_query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        let start = transform.translation;
        let velocity_unit = velocity.normalize();
        let end = Vec3::new(
            start.x + velocity_unit.x * 100.0,
            start.y + velocity_unit.y * 100.0,
            start.z,
        );
        let duration = 0.0;
        lines.line(start, end, duration);
    }
}

fn handle_bounce(
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut ball_velocity = ball_query.single_mut();

    if !collision_events.is_empty() {
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

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
    // reflect the ball when it collides
}
