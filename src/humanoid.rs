use std::time::{Duration, Instant};

use tetra::graphics::{animation::Animation, Texture};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::Context;

use crate::Direction;
use crate::BOUNDS;
use crate::{animation::HumanoidAnimation, RAD_TO_DEG};

pub enum HumanoidType {
    Player,
    BasicEnemy,
    StrongerEnemy,
    BadassEnemy,
    Boss,
}

pub struct Humanoid {
    pub hearts: u8,
    direction: Direction,
    animation: HumanoidAnimation,
    pub position: Vec2<f32>,
    velocity: Vec2<f32>,
    /// Set when the humanoid should 'flicker', such as when the player is hit
    pub flickering: u16,

    // TODO: these three shooting-related variables should belong to a struct/enum of their own
    /// Determines if the humanoid can shoot
    pub allowed_to_shoot: bool,
    /// The last moment that this humanoid shot a projectile   
    last_projectile_thrown_time: Instant,
    /// The interval in which this humanoid can shoot
    shooting_wait_time: Duration,
    kind: HumanoidType,
}

impl Humanoid {
    pub fn new(
        texture: Texture,
        position: Vec2<f32>,
        velocity: Vec2<f32>,
        allowed_to_shoot: bool,
        shooting_wait_time: Duration,
        kind: HumanoidType,
    ) -> Self {
        Self {
            hearts: 2,
            flickering: 0,
            direction: Direction::North,
            animation: HumanoidAnimation::new(texture),
            allowed_to_shoot,
            last_projectile_thrown_time: Instant::now(),
            shooting_wait_time,
            position,
            velocity,
            kind,
        }
    }

    pub fn can_fire(&self) -> bool {
        let time_since_last_show = self.last_projectile_thrown_time.elapsed();

        time_since_last_show >= self.shooting_wait_time
    }

    pub fn register_fire(&mut self) {
        self.last_projectile_thrown_time = Instant::now();
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
        match self.direction {
            Direction::North => (&self.animation.backside, scale),
            Direction::West => (&self.animation.leftside, scale),
            Direction::East => (&self.animation.leftside, Vec2 { x: -3., y: 3. }),
            Direction::South => (&self.animation.frontside, scale),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        if self.flickering > 0 {
            self.flickering -= 1;
            if self.flickering % 2 == 0 {
                return;
            }
        }

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
        const HERO_SPEED: f32 = 2.1;
        // Drag is only applied to the previous frame movement
        const HERO_MOVING_DRAG: f32 = 1.4;
        const HERO_STOPPING_DRAG: f32 = 1.9;

        let is_key_pressed_f32 = |key| input::is_key_down(ctx, key) as u8 as f32;
        // Movement for the axis x and y, can be -1, 0 or 1
        // We assume that 1.0 - 1.0 is always perfectly 0.0
        let x = is_key_pressed_f32(Key::D) - is_key_pressed_f32(Key::A);
        let y = is_key_pressed_f32(Key::S) - is_key_pressed_f32(Key::W);

        // Will be added to self.velocity
        let mut new_velocity = Vec2 { x, y };

        let dir = match (x as i32, y as i32) {
            (-1, 0) => Direction::West,
            (0, 1) => Direction::South,
            (1, 0) => Direction::East,
            (0, -1) => Direction::North,
            (1, 1) => Direction::South,
            (1, -1) => Direction::East,
            (_, _) => self.direction,
        };

        self.direction = dir;

        // Movement is in diagonal if both x and y contain non-zero values
        let is_diagonal = x != 0.0 && y != 0.0;
        // Moving in the diagonal shouldn't be faster than in vertical or horizontal, so we make
        // sure that the length of this Vec2 is always 1 (x² + y² == 1)
        // X and Y will equal to 0.707106...
        //
        // Need if because .normalize() on Vec2 {0, 0} is chaotic
        if is_diagonal {
            new_velocity.normalize();
        }

        // Apply drag.
        // This way of applying drag depends on the framerate, but that's not a huge problem,
        // because currently all our movement does.
        if new_velocity.magnitude() == 0.0 {
            // If no input was added, apply more drag to stop the hero
            self.velocity /= HERO_STOPPING_DRAG;
        } else {
            self.velocity /= HERO_MOVING_DRAG;
        }

        let new_pos = self.position + new_velocity + self.velocity * HERO_SPEED;

        if BOUNDS.contains(new_pos) {
            self.position = new_pos;
            self.velocity += new_velocity;
        }
    }

    pub fn look_to(&mut self, direction_deg: f32) {
        let direction = |mut angle: f32| -> i32 {
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
            _ => unreachable!(),
        }
    }

    pub fn angle_to_pos(&self, destination: Vec2<f32>) -> f32 {
        let pos = self.position;

        let delta_x = destination.x - pos.x;
        let delta_y = pos.y - destination.y;
        
        f32::atan2(delta_y, delta_x)
    }

    pub fn head_to(&mut self, destination: Vec2<f32>) {
        let theta_rad = self.angle_to_pos(destination);

        self.position += Vec2::new(f32::cos(theta_rad), -f32::sin(theta_rad)) * self.velocity;

        // Sets the Humanoid's Direction according to the calculated angle
        self.look_to(theta_rad * RAD_TO_DEG);
    }

    pub fn collided_with_bodies(&self, bodies: &[Humanoid]) -> (bool, Vec<Rectangle>) {
        let player_rect = Rectangle::new(self.position.x, self.position.y, 16.0, 16.0);
        let body_rects: Vec<_> = bodies
            .iter()
            .map(|e| e.position)
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
