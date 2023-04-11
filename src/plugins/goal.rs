pub struct GoalPlugin;

use crate::components::{Ball, CollisionEvent, GoalScoredEvent, PongSystemSet, Velocity};
use crate::constants::*;
use crate::helpers::random_direction;
use crate::PlayState;
use bevy::prelude::*;

use super::world_builder::BallSpeed;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                check_for_goals.run_if(on_event::<CollisionEvent>()),
                reset_ball_on_goal
                    .after(check_for_goals)
                    .run_if(on_event::<GoalScoredEvent>()),
            )
                .in_set(PongSystemSet::GameLogic),
        )
        .add_event::<GoalScoredEvent>();
    }
}

fn check_for_goals(
    mut goal_scored_event_writer: EventWriter<GoalScoredEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    mut game_state: ResMut<NextState<PlayState>>,
) {
    for collision_event in collision_events.iter() {
        if let Some(player) = collision_event.player_goal {
            goal_scored_event_writer.send(GoalScoredEvent { player });
            game_state.set(PlayState::Paused);
        }
    }
}

fn reset_ball_on_goal(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut ball_speed: ResMut<BallSpeed>,
) {
    for (mut transform, mut velocity) in ball_query.iter_mut() {
        transform.translation = BALL_START_POSITION;
        velocity.0 = random_direction() * INITIAL_BALL_SPEED;
        ball_speed.0 = INITIAL_BALL_SPEED
    }
}
