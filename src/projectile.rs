use core::f32;

use tetra::{
    graphics::{animation::Animation, DrawParams},
    math::Vec2,
    Context,
};

use crate::{traits::Cleanable, BOUNDS};

#[derive(Clone)]
pub struct Projectile {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    angle_rad: f32,
}

impl Projectile {
    pub fn position(&self) -> Vec2<f32> {
        self.position
    }
}

pub struct ProjectileManager {
    projectiles: Vec<Projectile>,
    animation: Animation,
}

impl Cleanable for ProjectileManager {
    /// Remove projectiles that have gone out of bounds
    fn clean_up(&mut self) {
        let is_in_bounds = |fireball: &Projectile| {
            BOUNDS.contains(fireball.position())
        };

        self.projectiles.retain(is_in_bounds);
    }
}

impl ProjectileManager {
    pub fn new(animation: Animation) -> Self {
        Self {
            projectiles: Vec::with_capacity(48),
            animation,
        }
    }

    pub fn shoot(
        &mut self,
        is_triple_shooting: bool,
        angle: f32,
        position: Vec2<f32>,
        velocity: Vec2<f32>,
    ) {
        if is_triple_shooting {
            self.send_triple_shot(angle, position, velocity);
        } else {
            self.add_projectile(angle, position, velocity)
        }
    }

    fn send_triple_shot(
        &mut self,
        angle: f32,
        position: Vec2<f32>,
        velocity: Vec2<f32>,
    ) {
        for deviation in [-45.0, 0.0, 45.0] {
            self.add_projectile(
                angle + deviation,
                position,
                velocity,
            );
        }
    }

    /// Adds a projectile to this `ProjectileManager`.
    /// The angle supplied should be in degrees.
    pub fn add_projectile(
        &mut self,
        angle: f32,
        position: Vec2<f32>,
        velocity: Vec2<f32>,
    ) {
        let angle_rad = angle.to_radians();

        let fireball = Projectile {
            position,
            angle_rad,
            velocity,
        };

        self.projectiles.push(fireball);
    }

    pub fn advance_animation(&mut self, ctx: &mut Context) {
        self.animation.advance(ctx);

        self.clean_up();

        for fireball in &mut self.projectiles {
            let ang_rad = fireball.angle_rad;
            fireball.position +=
                Vec2::new(f32::cos(ang_rad), -f32::sin(ang_rad))
                    * fireball.velocity;
        }
    }

    pub fn projectiles(&self) -> &[Projectile] {
        &self.projectiles
    }

    pub fn draw(&self, ctx: &mut Context) {
        for fireball in &self.projectiles {
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
