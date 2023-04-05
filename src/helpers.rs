use rand::prelude::*;

use bevy::prelude::*;

pub fn random_direction() -> Vec2 {
    let mut rng = rand::thread_rng();

    let x_speed = rng.gen_range(1.0..2.0);
    let move_right = rng.gen_bool(0.5);
    let x_speed = if move_right { x_speed } else { -x_speed };

    let y_speed = rng.gen_range(1.0..2.0);
    let move_up = rng.gen_bool(0.5);
    let y_speed = if move_up { y_speed } else { -y_speed };

    Vec2::new(x_speed, y_speed).normalize()
}
