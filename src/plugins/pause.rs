pub struct PausePlugin;

use crate::components::PauseText;
use bevy::prelude::*;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayState>()
            .add_system(handle_unpause.in_set(OnUpdate(PlayState::Paused)))
            .add_system(show_pause_text.in_schedule(OnEnter(PlayState::Paused)))
            .add_system(hide_pause_text.in_schedule(OnExit(PlayState::Paused)));
    }
}

#[derive(Debug, Hash, Copy, Clone, Resource, Eq, PartialEq, Default, States)]
pub enum PlayState {
    Playing,
    #[default]
    Paused,
}

fn handle_unpause(
    keyboard_input: Res<Input<KeyCode>>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        play_state.set(PlayState::Playing);
    }
}

fn show_pause_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Press Space to start",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..Default::default()
        },
        PauseText,
    ));
}

fn hide_pause_text(mut commands: Commands, pause_text_query: Query<Entity, With<PauseText>>) {
    for entity in pause_text_query.iter() {
        commands.entity(entity).despawn();
    }
}
