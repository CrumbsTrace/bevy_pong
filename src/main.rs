mod components;
mod constants;
mod helpers;
mod paddle_bundle;
mod plugins;
mod wall_bundle;

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use constants::RESOLUTION;
use plugins::{
    CollisionPlugin, GoalPlugin, MovementPlugin, PausePlugin, PlayState, ScoreboardPlugin,
    WorldBuilderPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: RESOLUTION.into(),
                title: "Bevy Pong".to_string(),
                ..default()
            }),
            ..default()
        }))
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(WorldBuilderPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GoalPlugin)
        .add_plugin(ScoreboardPlugin)
        .insert_resource(ClearColor(constants::BACKGROUND_COLOR))
        .add_system(bevy::window::close_on_esc)
        .run();
}
