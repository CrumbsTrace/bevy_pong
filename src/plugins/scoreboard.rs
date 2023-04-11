use crate::components::{GoalScoredEvent, Player, PongSystemSet, ScoreboardText};
use crate::constants::*;
use bevy::prelude::*;

pub struct ScoreboardPlugin;

#[derive(Resource)]
struct Scoreboard {
    left: usize,
    right: usize,
}

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_scoreboard)
            .insert_resource(Scoreboard { left: 0, right: 0 })
            .add_system(
                update_scoreboard
                    .in_set(PongSystemSet::GameLogic)
                    .run_if(on_event::<GoalScoredEvent>()),
            );
    }
}

fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scoreboard_position = Vec3::new(0.0, TOP_WALL + 50.0, 0.0);
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0 - 0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_TEXT_COLOR,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(scoreboard_position),
            ..default()
        },
        ScoreboardText,
    ));
}

fn update_scoreboard(
    mut scoreboard_text_query: Query<&mut Text, With<ScoreboardText>>,
    mut goal_scored_events: EventReader<GoalScoredEvent>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let mut scoreboard_text = scoreboard_text_query.single_mut();

    for event in goal_scored_events.iter() {
        match event.player {
            Player::Right => scoreboard.left += 1,
            Player::Left => scoreboard.right += 1,
        }
    }

    scoreboard_text.sections[0].value = format!("{} - {}", scoreboard.left, scoreboard.right);
}
