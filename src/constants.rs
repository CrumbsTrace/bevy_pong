use bevy::prelude::*;

pub const TIME_STEP: f32 = 1.0 / 60.0;

//Pong field definitions
pub const LEFT_WALL: f32 = -450.0;
pub const RIGHT_WALL: f32 = 450.0;
pub const TOP_WALL: f32 = 300.0;
pub const BOTTOM_WALL: f32 = -300.0;
pub const WALL_THICKNESS: f32 = 20.0;

//Paddle definitions
pub const PADDLE_SIZE: Vec3 = Vec3::new(20.0, 100.0, 1.0);
pub const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_WALL_GAP: f32 = 5.0;
pub const LEFT_PADDLE_POSITION: Vec2 = Vec2::new(LEFT_WALL + WALL_THICKNESS + PADDLE_WALL_GAP, 0.0);
pub const RIGHT_PADDLE_POSITION: Vec2 =
    Vec2::new(RIGHT_WALL - WALL_THICKNESS - PADDLE_WALL_GAP, 0.0);

//Ball definitions
pub const BALL_START_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const BALL_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
pub const BALL_SPEED: f32 = 600.0;

//Color definitions
pub const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);
pub const PADDLE_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

pub const SCOREBOARD_TEXT_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const SCOREBOARD_FONT_SIZE: f32 = 40.0;
