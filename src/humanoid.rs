use tetra::graphics::DrawParams;
use tetra::graphics::{animation::Animation, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::Context;

use crate::animation::HumanoidAnimation;
use crate::Direction;

pub struct Humanoid {
    health_points: u16,
    direction: Direction,
    animation: HumanoidAnimation,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Humanoid {
    pub fn new(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Self {
        Self {
            health_points: 100,
            direction: Direction::North,
            animation: HumanoidAnimation::new(texture),
            position,
            velocity,
        }
    }

    pub fn advance_animation(&mut self, ctx: &mut Context) {
        match self.direction {
            Direction::North => self.animation.backside.advance(ctx),
            Direction::West | Direction::East => self.animation.leftside.advance(ctx),
            Direction::South => self.animation.frontside.advance(ctx),
        }
    }

    fn get_animation_ref(&self) -> (&Animation, Vec2<f32>) {
        let scale = Vec2::new(3., 3.);
        let scale_reverse = Vec2::new(-3., 3.);
        match self.direction {
            Direction::North => (&self.animation.backside, scale),
            Direction::West => (&self.animation.leftside, scale),
            Direction::East => (&self.animation.leftside, scale_reverse),
            Direction::South => (&self.animation.frontside, scale),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let (animation, scale) = self.get_animation_ref();

        animation.draw(
            ctx,
            DrawParams::new()
                .position(self.position)
                .origin(Vec2::new(8.0, 8.0))
                .scale(scale),
        );
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if input::is_key_down(ctx, Key::A) {
            self.velocity.x = (self.velocity.x - 0.5).max(-5.0);
            self.set_direction(Direction::West);
        } else if input::is_key_down(ctx, Key::S) {
            self.velocity.y = (self.velocity.y + 0.5).min(5.0);
            self.set_direction(Direction::South);
        } else if input::is_key_down(ctx, Key::D) {
            self.velocity.x = (self.velocity.x + 0.5).min(5.0);
            self.set_direction(Direction::East);
        } else if input::is_key_down(ctx, Key::W) {
            self.velocity.y = (self.velocity.y - 0.5).max(-5.0);
            self.set_direction(Direction::North);
        } else {
            self.velocity.x -= self.velocity.x.abs().min(0.5) * self.velocity.x.signum();
            self.velocity.y -= self.velocity.y.abs().min(0.5) * self.velocity.y.signum();
        }

        self.position += self.velocity;
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn get_position(&self) -> Vec2<f32> {
        self.position
    }
}
