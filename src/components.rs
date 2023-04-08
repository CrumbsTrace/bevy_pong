use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Clone, Copy, Debug)]
pub enum Player {
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct SideWall {
    pub player: Player,
}

pub struct GoalScoredEvent {
    pub player: Player,
}

pub struct CollisionEvent {
    pub collision: Collision,
    pub player_goal: Option<Player>,
}

#[derive(Component)]
pub struct MovementKeys {
    pub up: KeyCode,
    pub down: KeyCode,
}

#[derive(Component)]
pub struct PauseText;
