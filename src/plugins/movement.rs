pub struct MovementPlugin;

use crate::components::{MovementKeys, Velocity};
use crate::constants::*;
use crate::GameState;

use bevy::prelude::*;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((move_paddles, move_ball).in_schedule(CoreSchedule::FixedUpdate));
    }
}

fn move_paddles(
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

        let new_position = transform.translation.y + direction * PADDLE_SPEED * TIME_STEP;

        //Clamp paddle position
        let top_bound = TOP_WALL - PADDLE_SIZE.y / 2.0 - WALL_THICKNESS / 2.0;
        let bottom_bound = BOTTOM_WALL + PADDLE_SIZE.y / 2.0 + WALL_THICKNESS / 2.0;

        transform.translation.y = new_position.clamp(bottom_bound, top_bound);
    }
}

fn move_ball(mut ball_query: Query<(&mut Transform, &Velocity)>, game_state: Res<GameState>) {
    if game_state.is_paused() {
        return;
    }

    for (mut transform, velocity) in ball_query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}
