pub struct PausePlugin;

use crate::components::PauseText;
use bevy::prelude::*;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((handle_play_state, show_pause_text.after(handle_play_state)))
            .insert_resource(PlayState::Paused);
    }
}

#[derive(Resource, Eq, PartialEq)]
pub enum PlayState {
    Playing,
    Paused,
}

impl PlayState {
    pub fn is_paused(&self) -> bool {
        *self == PlayState::Paused
    }
}

fn handle_play_state(keyboard_input: Res<Input<KeyCode>>, mut play_state: ResMut<PlayState>) {
    if play_state.is_paused() && keyboard_input.just_pressed(KeyCode::Space) {
        *play_state = PlayState::Playing;
    }
}

fn show_pause_text(
    mut commands: Commands,
    game_state: Res<PlayState>,
    asset_server: Res<AssetServer>,
    pause_text_query: Query<Entity, With<PauseText>>,
) {
    if game_state.is_changed() && game_state.is_paused() {
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
    } else if game_state.is_changed() && !game_state.is_paused() {
        for entity in pause_text_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
