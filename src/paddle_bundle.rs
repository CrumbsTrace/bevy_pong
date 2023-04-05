use crate::components::*;
use crate::constants::{LEFT_PADDLE_POSITION, PADDLE_COLOR, PADDLE_SIZE, RIGHT_PADDLE_POSITION};
use bevy::prelude::*;

pub enum PaddleInfo {
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
pub struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    movement_keys: MovementKeys,
}

impl PaddleBundle {
    pub fn new(info: PaddleInfo) -> Self {
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
