pub struct GoalPlugin;

use crate::components::{Ball, CollisionEvent, GoalScoredEvent, Velocity};
use crate::constants::*;
use crate::helpers::random_direction;
use crate::PlayState;
use bevy::prelude::*;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((check_for_goals, reset_ball_on_goal.after(check_for_goals)))
            .add_event::<GoalScoredEvent>();
    }
}

fn check_for_goals(
    mut goal_scored_event_writer: EventWriter<GoalScoredEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    mut game_state: ResMut<PlayState>,
) {
    for collision_event in collision_events.iter() {
        if let Some(player) = collision_event.player_goal {
            goal_scored_event_writer.send(GoalScoredEvent { player });
            *game_state = PlayState::Paused;
        }
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
