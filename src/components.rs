use crate::paddle_bundle::PaddleInfo;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

pub struct GoalScoredEvent {
    pub player: PaddleInfo,
}

pub struct CollisionEvent {
    pub collision: Collision,
}

#[derive(Component)]
pub struct MovementKeys {
    pub up: KeyCode,
    pub down: KeyCode,
}

#[derive(Component)]
pub struct PauseText;
