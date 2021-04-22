use std::collections::HashMap;

use tetra::{Context, graphics::{DrawParams, Rectangle, Texture, animation::Animation}, math::Vec2};

use crate::{DEG_TO_RAD, animation::FireballAnimation, sprites::FIREBALL};

#[derive(Clone)]
pub struct Fireball {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    angle_rad: f32,
}

impl Fireball {
    pub fn out_of_bounds(pos: Vec2<f32>) -> bool {
        pos.x > 800. || pos.y > 800. || pos.x < 0. || pos.y < 0.
    }
}

pub struct FireballManager {
    // LinkedList would be better here but
    // linked_list_remove is currently unstable (as of rustc 1.51)
    fireballs: Vec<Fireball>,
    animation: Animation,
}

impl FireballManager {
    pub fn new(ctx: &mut Context) -> Self {
        let texture = Texture::from_file_data(ctx, FIREBALL).expect("couldn't read the fireball sprite");
        let animation = FireballAnimation::make_animation(texture);

        Self {
            fireballs: vec![],
            animation,
        }
    }

    pub fn add_fireball(&mut self, angle: f32, position: Vec2<f32>) {
        let angle_rad = angle * DEG_TO_RAD;

        // We'll start the fireball 5 units away from the player in the given direction
        let position = position + Vec2::new(f32::sin(angle_rad) * 5., f32::cos(angle_rad) * 5.);
        let fireball = Fireball {
            position,
            angle_rad,
            velocity: Vec2::new(5., 5.)
        };

        self.fireballs.push(fireball);
    }

    // This is slow, I guess, (and ugly) but any other solution I can think of right now gets into ownership hell
    pub fn clean_up_oob(&mut self) {
        let mut indices_to_remove = HashMap::new();

        for (idx, fireball) in self.fireballs.iter().enumerate() {
            let pos = fireball.position;
            if Fireball::out_of_bounds(pos) {
                indices_to_remove.insert(idx, true);
            }
        }

        if indices_to_remove.is_empty() {
            return;
        }

        let mut fireballs = vec![];
        for idx in 0..self.fireballs.len() {
            if let Some(true) = indices_to_remove.get(&idx) {
                continue;
            }
            fireballs.push(self.fireballs[idx].clone());
        }

        self.fireballs = fireballs;
    }

    pub fn advance_animation(&mut self, ctx: &mut Context) {
        self.animation.advance(ctx);

        self.clean_up_oob();

        for fireball in &mut self.fireballs {
            let ang_rad = fireball.angle_rad;
            fireball.position += Vec2::new(f32::sin(ang_rad), f32::cos(ang_rad)) * fireball.velocity;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for fireball in &self.fireballs {
            self.animation.draw(ctx, DrawParams::new().position(fireball.position))
        }
    }
}