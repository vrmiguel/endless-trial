use core::f32;

use tetra::{
    graphics::{animation::Animation, DrawParams},
    math::Vec2,
    Context,
};

use crate::{BOUNDS, DEG_TO_RAD};

#[derive(Clone)]
pub struct Fireball {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    angle_rad: f32,
}

impl Fireball {
    pub fn get_position(&self) -> Vec2<f32> {
        self.position
    }
}

pub struct ProjectileManager {
    fireballs: Vec<Fireball>,
    animation: Animation,
}

impl ProjectileManager {
    pub fn new(animation: Animation) -> Self {
        Self {
            fireballs: vec![],
            animation,
        }
    }

    pub fn add_projectile(&mut self, angle: f32, position: Vec2<f32>) {
        let angle_rad = angle * DEG_TO_RAD;

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
            .retain(|fireball| BOUNDS.contains(fireball.get_position()));
    }

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
                    .origin(Vec2::new(16.0, 16.0))
                    .rotation(fireball.angle_rad),
            )
        }
    }
}
