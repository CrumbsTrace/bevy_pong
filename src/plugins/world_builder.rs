pub struct WorldBuilderPlugin;

use crate::components::{Ball, Player, SideWall, Velocity};
use crate::constants::*;
use crate::helpers::random_direction;
use crate::paddle_bundle::{PaddleBundle, PaddleInfo};
use crate::wall_bundle::{WallBundle, WallLocation};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Resource)]
pub struct BallSpeed(pub f32);

impl Plugin for WorldBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(BallSpeed(INITIAL_BALL_SPEED));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    //Paddles
    commands.spawn(PaddleBundle::new(PaddleInfo::Left));
    commands.spawn(PaddleBundle::new(PaddleInfo::Right));

    //Walls
    commands.spawn((
        WallBundle::new(WallLocation::Left),
        SideWall {
            player: Player::Left,
        },
    ));

    commands.spawn((
        WallBundle::new(WallLocation::Right),
        SideWall {
            player: Player::Right,
        },
    ));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    //Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(BALL_COLOR.into()),
            transform: Transform::from_translation(BALL_START_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(random_direction() * INITIAL_BALL_SPEED),
    ));
}
