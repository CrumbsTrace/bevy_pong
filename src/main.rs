use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

use rand::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

//Pong field definitions
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const TOP_WALL: f32 = 300.0;
const BOTTOM_WALL: f32 = -300.0;
const WALL_THICKNESS: f32 = 20.0;

//Paddle definitions
const PADDLE_SIZE: Vec3 = Vec3::new(20.0, 100.0, 1.0);
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_WALL_GAP: f32 = 5.0;
const LEFT_PADDLE_POSITION: Vec2 = Vec2::new(LEFT_WALL + WALL_THICKNESS + PADDLE_WALL_GAP, 0.0);
const RIGHT_PADDLE_POSITION: Vec2 = Vec2::new(RIGHT_WALL - WALL_THICKNESS - PADDLE_WALL_GAP, 0.0);

//Ball definitions
const BALL_START_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const BALL_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const BALL_SPEED: f32 = 600.0;

//Color definitions
const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);
const PADDLE_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

const SCOREBOARD_TEXT_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const SCOREBOARD_FONT_SIZE: f32 = 40.0;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct MovementKeys {
    up: KeyCode,
    down: KeyCode,
}

struct GoalScoredEvent {
    player: PaddleInfo,
}

enum PaddleInfo {
    Left,
    Right,
}

impl PaddleInfo {
    fn position(&self) -> Vec3 {
        match self {
            PaddleInfo::Left => LEFT_PADDLE_POSITION.extend(0.0),
            PaddleInfo::Right => RIGHT_PADDLE_POSITION.extend(0.0),
        }
    }

    fn size(&self) -> Vec3 {
        PADDLE_SIZE
    }

    fn key_up(&self) -> KeyCode {
        match self {
            PaddleInfo::Left => KeyCode::W,
            PaddleInfo::Right => KeyCode::Up,
        }
    }

    fn key_down(&self) -> KeyCode {
        match self {
            PaddleInfo::Left => KeyCode::S,
            PaddleInfo::Right => KeyCode::Down,
        }
    }
}

#[derive(Bundle)]
struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    movement_keys: MovementKeys,
}

impl PaddleBundle {
    fn new(info: PaddleInfo) -> Self {
        PaddleBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: info.position(),
                    scale: info.size(),
                    ..default()
                },
                sprite: Sprite {
                    color: PADDLE_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
            movement_keys: MovementKeys {
                up: info.key_up(),
                down: info.key_down(),
            },
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(paddle_info: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: paddle_info.position().extend(0.0),
                    scale: paddle_info.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

#[derive(Resource)]
struct Scoreboard {
    left: usize,
    right: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Scoreboard { left: 0, right: 0 })
        .add_startup_system(setup)
        .add_event::<GoalScoredEvent>()
        .add_systems(
            (
                check_collisions,
                move_ball.before(check_collisions),
                move_paddles.before(check_collisions).after(move_ball),
                check_for_goals.after(move_ball),
                reset_ball_on_goal.after(check_for_goals),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    //Paddles
    commands.spawn(PaddleBundle::new(PaddleInfo::Left));
    commands.spawn(PaddleBundle::new(PaddleInfo::Right));

    //Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
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
        Velocity(random_direction() * BALL_SPEED),
    ));

    let scoreboard_position = Vec3::new(0.0, TOP_WALL + 50.0, 0.0);
    commands.spawn(Text2dBundle {
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
    });
}

fn random_direction() -> Vec2 {
    let mut rng = rand::thread_rng();

    let x_speed = rng.gen_range(1.0..2.0);
    let move_right = rng.gen_bool(0.5);
    let x_speed = if move_right { x_speed } else { -x_speed };

    let y_speed = rng.gen_range(1.0..2.0);
    let move_up = rng.gen_bool(0.5);
    let y_speed = if move_up { y_speed } else { -y_speed };

    Vec2::new(x_speed, y_speed).normalize()
}

fn move_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &MovementKeys)>,
) {
    for (mut transform, movement_keys) in query.iter_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(movement_keys.up) {
            direction += 1.0;
        }
        if keyboard_input.pressed(movement_keys.down) {
            direction -= 1.0;
        }

        let new_position = transform.translation.y + direction * PADDLE_SPEED * TIME_STEP;

        //Clamp paddle position
        let top_bound = TOP_WALL - PADDLE_SIZE.y / 2.0 - WALL_THICKNESS / 2.0;
        let bottom_bound = BOTTOM_WALL + PADDLE_SIZE.y / 2.0 + WALL_THICKNESS / 2.0;

        transform.translation.y = new_position.clamp(bottom_bound, top_bound);
    }
}

fn move_ball(mut ball_query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in ball_query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn check_collisions(
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (ball_transform, mut ball_velocity) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for collider_transform in collider_query.iter() {
        let collider_size = collider_transform.scale.truncate();

        let collision = collide(
            ball_transform.translation,
            ball_size,
            collider_transform.translation,
            collider_size,
        );

        if let Some(collision) = collision {
            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn check_for_goals(
    mut goal_scored_event_writer: EventWriter<GoalScoredEvent>,
    mut ball_query: Query<&Transform, With<Ball>>,
) {
    let ball_transform = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    let left_bound = LEFT_PADDLE_POSITION.x - PADDLE_SIZE.x / 2.0;
    let right_bound = RIGHT_PADDLE_POSITION.x + PADDLE_SIZE.x / 2.0;

    if ball_transform.translation.x - ball_size.x / 2.0 < left_bound {
        goal_scored_event_writer.send(GoalScoredEvent {
            player: PaddleInfo::Right,
        });
    } else if ball_transform.translation.x + ball_size.x / 2.0 > right_bound {
        goal_scored_event_writer.send(GoalScoredEvent {
            player: PaddleInfo::Left,
        });
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

fn update_scoreboard(
    mut scoreboard_text_query: Query<&mut Text>,
    mut goal_scored_events: EventReader<GoalScoredEvent>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    if !goal_scored_events.is_empty() {
        let mut scoreboard_text = scoreboard_text_query.single_mut();

        for event in goal_scored_events.iter() {
            match event.player {
                PaddleInfo::Left => scoreboard.left += 1,
                PaddleInfo::Right => scoreboard.right += 1,
            }
        }

        scoreboard_text.sections[0].value = format!("{} - {}", scoreboard.left, scoreboard.right);
    }
}
