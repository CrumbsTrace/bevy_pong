mod components;
mod constants;
mod helpers;
mod paddle_bundle;
mod plugins;
mod wall_bundle;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use plugins::{CollisionPlugin, GoalPlugin, MovementPlugin, ScoreboardPlugin, WorldBuilderPlugin};

#[derive(Resource, Eq, PartialEq)]
enum GameState {
    Playing,
    Paused,
}

impl GameState {
    pub fn is_paused(&self) -> bool {
        *self == GameState::Paused
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(WorldBuilderPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GoalPlugin)
        .add_plugin(ScoreboardPlugin)
        .insert_resource(ClearColor(constants::BACKGROUND_COLOR))
        .insert_resource(GameState::Paused)
        .add_system(handle_game_state)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn handle_game_state(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match *game_state {
            GameState::Playing => *game_state = GameState::Paused,
            GameState::Paused => *game_state = GameState::Playing,
        }
    }
}
