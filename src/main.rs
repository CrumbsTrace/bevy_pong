mod components;
mod constants;
mod helpers;
mod paddle_bundle;
mod plugins;
mod wall_bundle;

use bevy::prelude::*;

use plugins::{CollisionPlugin, GoalPlugin, MovementPlugin, ScoreboardPlugin, WorldBuilderPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldBuilderPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GoalPlugin)
        .add_plugin(ScoreboardPlugin)
        .insert_resource(ClearColor(constants::BACKGROUND_COLOR))
        .add_system(bevy::window::close_on_esc)
        .run();
}
