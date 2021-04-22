use tetra::{Context, graphics::{Texture, animation::Animation}, math::Vec2};

use crate::{animation, sprites::FIREBALL};

pub struct Fireball {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    angle: f32,
}

pub struct FireballManager {
    fireballs: Vec<Fireball>,
    animation: animation::FireballAnimation,
}

impl FireballManager {
    pub fn new(ctx: &mut Context) -> Self {
        let texture = Texture::from_file_data(ctx, FIREBALL).expect("couldn't read the fireball sprite");
        let animation = animation::FireballAnimation::new(texture);

        Self {
            fireballs: vec![],
            animation
        }
    }
}