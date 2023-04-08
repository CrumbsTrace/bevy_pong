pub struct CollisionPlugin;

use crate::components::{Ball, CollisionEvent};
use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::components::Collider;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_collisions)
            .add_event::<CollisionEvent>();
    }
}

fn check_collisions(
    mut ball_query: Query<&Transform, With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    let ball_transform = ball_query.single_mut();
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
            collision_event.send(CollisionEvent { collision });
        }
    }
}
