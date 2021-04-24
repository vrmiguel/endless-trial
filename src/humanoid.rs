use tetra::graphics::{DrawParams, Rectangle};
use tetra::graphics::{animation::Animation, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::Context;

use crate::{RAD_TO_DEG, animation::HumanoidAnimation};
use crate::Direction;

pub enum HumanoidType {
    Player,
    BasicEnemy,
    StrongerEnemy,
    BadassEnemy,
    Boss,
}

pub struct Humanoid {
    health_points: u16,
    direction: Direction,
    animation: HumanoidAnimation,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    kind: HumanoidType,
}

impl Humanoid {
    pub fn new(
        texture: Texture,
        position: Vec2<f32>,
        velocity: Vec2<f32>,
        kind: HumanoidType,
    ) -> Self {
        Self {
            health_points: 100,
            direction: Direction::North,
            animation: HumanoidAnimation::new(texture),
            position,
            velocity,
            kind,
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

    pub fn update_from_key_press(&mut self, ctx: &mut Context) {
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

    pub fn look_to(&mut self, direction_deg: f32) {

        let direction = |mut angle: f32|-> i32 {
            if angle < 0. {
                angle += 360.;
            }
            let angle = angle as i32;
            (45 + angle) % 360 / 90
        };
        
        self.direction = match direction(direction_deg) {
            0 => Direction::East,
            1 => Direction::North,
            2 => Direction::West,
            3 => Direction::South,
            _ => unreachable!()
        }
    }

    pub fn head_to(&mut self, destination: Vec2<f32>) {
        
        let old_pos = self.position;

        let delta_x = destination.x - old_pos.x;
        let delta_y = old_pos.y - destination.y;
        let theta_rad = f32::atan2(delta_y, delta_x);
        
        self.position +=
                Vec2::new(f32::cos(theta_rad), -f32::sin(theta_rad)) * self.velocity;
        
        self.look_to(theta_rad * RAD_TO_DEG);
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn get_position(&self) -> Vec2<f32> {
        self.position
    }

    pub fn set_position(&mut self, new_pos: Vec2<f32>) {
        self.position = new_pos;
    }

    pub fn collided_with_bodies(&self, bodies: &[Humanoid]) -> (bool, Vec<Rectangle>) {
        let player_rect = Rectangle::new(self.position.x, self.position.y, 16.0, 16.0);
        let body_rects: Vec<_> = bodies
            .iter()
            .map(|e| e.get_position())
            .map(Vec2::into_tuple)
            .map(|(x, y)| Rectangle::new(x, y, 16.0, 16.0))
            .collect();
        for body_rect in &body_rects {
            if player_rect.intersects(&body_rect) {
                return (true, body_rects);
            }
        }
        (false, body_rects)
    }
}
