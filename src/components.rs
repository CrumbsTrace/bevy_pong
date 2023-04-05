use crate::paddle_bundle::PaddleInfo;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

pub struct GoalScoredEvent {
    pub player: PaddleInfo,
}

#[derive(Component)]
pub struct MovementKeys {
    pub up: KeyCode,
    pub down: KeyCode,
}
