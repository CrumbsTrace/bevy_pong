pub struct CollisionPlugin;

use crate::components::{Ball, Velocity};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::components::Collider;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_collisions);
    }
}

fn check_collisions(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (ball_transform, mut ball_velocity) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for collider_transform in collider_query.iter() {
        let collider_size = collider_transform.scale.truncate();

        let collision = collide(
            ball_transform.translation,
            ball_size,
            collider_transform.translation,
            collider_size,
        );

        if let Some(collision) = collision {
            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
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
}
