pub struct GoalPlugin;

use crate::components::{Ball, GoalScoredEvent, Velocity};
use crate::constants::*;
use crate::helpers::random_direction;
use crate::paddle_bundle::PaddleInfo;
use crate::GameState;
use bevy::prelude::*;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((check_for_goals, reset_ball_on_goal.after(check_for_goals)))
            .add_event::<GoalScoredEvent>();
    }
}

fn check_for_goals(
    mut goal_scored_event_writer: EventWriter<GoalScoredEvent>,
    mut ball_query: Query<&Transform, With<Ball>>,
    mut game_state: ResMut<GameState>,
) {
    let ball_transform = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    let left_bound = LEFT_PADDLE_POSITION.x - PADDLE_SIZE.x / 2.0;
    let right_bound = RIGHT_PADDLE_POSITION.x + PADDLE_SIZE.x / 2.0;

    if ball_transform.translation.x - ball_size.x / 2.0 < left_bound {
        goal_scored_event_writer.send(GoalScoredEvent {
            player: PaddleInfo::Right,
        });
        *game_state = GameState::Paused;
    } else if ball_transform.translation.x + ball_size.x / 2.0 > right_bound {
        goal_scored_event_writer.send(GoalScoredEvent {
            player: PaddleInfo::Left,
        });
        *game_state = GameState::Paused;
    }
}

fn reset_ball_on_goal(
    mut goal_scored_events: EventReader<GoalScoredEvent>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    if !goal_scored_events.is_empty() {
        goal_scored_events.clear();
        for (mut transform, mut velocity) in ball_query.iter_mut() {
            transform.translation = BALL_START_POSITION;
            velocity.0 = random_direction() * BALL_SPEED;
        }
    }
}
