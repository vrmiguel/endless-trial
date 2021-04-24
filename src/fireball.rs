use core::f32;
use std::time::Instant;
use std::{collections::HashMap, time::Duration};

use tetra::{
    graphics::{animation::Animation, DrawParams, Texture},
    math::Vec2,
    Context,
};

use crate::{animation::FireballAnimation, sprites::FIREBALL, DEG_TO_RAD};

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

    pub fn get_position(&self) -> Vec2<f32> {
        self.position
    }
}

pub struct FireballManager {
    // LinkedList would be better here but
    // linked_list_remove is currently unstable (as of rustc 1.51)
    fireballs: Vec<Fireball>,
    animation: Animation,
    last_thrown_time: Instant,
}

impl FireballManager {
    pub fn new(ctx: &mut Context) -> Self {
        let texture =
            Texture::from_file_data(ctx, FIREBALL).expect("couldn't read the fireball sprite");
        let animation = FireballAnimation::make_animation(texture);

        Self {
            fireballs: vec![],
            animation,
            last_thrown_time: std::time::Instant::now(),
        }
    }

    pub fn can_throw(&self) -> bool {
        let time_since_last_throw = self.last_thrown_time.elapsed();

        time_since_last_throw > Duration::from_secs_f64(0.25)
    }

    pub fn add_fireball(&mut self, angle: f32, position: Vec2<f32>) {
        let angle_rad = angle * DEG_TO_RAD;
        self.last_thrown_time = Instant::now();

        // We'll start the fireball 5 units away from the player in the given direction
        let position = position + Vec2::new(f32::cos(angle_rad), -f32::sin(angle_rad));
        let fireball = Fireball {
            position,
            angle_rad,
            velocity: Vec2::new(5., 5.),
        };

        self.fireballs.push(fireball);
    }

    pub fn clean_up_oob(&mut self) {
        self.fireballs
            .retain(|fireball| Fireball::out_of_bounds(fireball.position));
    }

    // TODO: rename to update?
    pub fn advance_animation(&mut self, ctx: &mut Context) {
        self.animation.advance(ctx);

        self.clean_up_oob();

        for fireball in &mut self.fireballs {
            let ang_rad = fireball.angle_rad;
            fireball.position +=
                Vec2::new(f32::cos(ang_rad), -f32::sin(ang_rad)) * fireball.velocity;
        }
    }

    pub fn fireballs_ref(&self) -> &[Fireball] {
        &self.fireballs
    }

    pub fn draw(&self, ctx: &mut Context) {
        for fireball in &self.fireballs {
            self.animation.draw(
                ctx,
                DrawParams::new()
                    .position(fireball.position)
                    .scale(Vec2::new(0.5, 0.5))
                    .rotation(fireball.angle_rad),
            )
        }
    }
}
