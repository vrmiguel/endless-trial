use std::time::{Duration, Instant};

use rand::{
    distributions::Standard,
    prelude::{Distribution, StdRng},
    Rng, SeedableRng,
};
use tetra::{
    graphics::{DrawParams, Texture},
    math::Vec2,
    Context,
};

use crate::sprites;

pub enum PowerUpKind {
    AdditionalHeart,
    FasterShooting,
}
struct PowerUp {
    kind: PowerUpKind,
    spawned_time: Instant,
    position: Vec2<f32>,
}

impl PowerUp {
    pub fn is_expired(&self) -> bool {
        let elapsed = self.spawned_time.elapsed();

        elapsed > Duration::from_secs_f32(10.0)
    }
}

impl Distribution<PowerUpKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUpKind {
        match rng.gen_range(0..=1) {
            0 => PowerUpKind::AdditionalHeart,
            _ => PowerUpKind::FasterShooting,
        }
    }
}

pub struct PowerUpManager {
    fire_scroll_sprite: Texture,
    heart_sprite: Texture,
    powerups: Vec<PowerUp>,
    last_spawned_time: Instant,
}

impl PowerUpManager {
    pub fn new(ctx: &mut Context) -> Self {
        let strawberry_sprite = Texture::from_file_data(ctx, sprites::FIRE_SCROLL)
            .expect("failed to load built-in strawberry sprite");
        let heart_sprite = Texture::from_file_data(ctx, sprites::HEART_32X)
            .expect("failed to load built-in heart 32x32 sprite");

        Self {
            fire_scroll_sprite: strawberry_sprite,
            heart_sprite,
            powerups: vec![],
            last_spawned_time: Instant::now(),
        }
    }

    pub fn can_spawn(&self) -> bool {
        let time_since_last_throw = self.last_spawned_time.elapsed();
        time_since_last_throw > Duration::from_secs_f64(5.00)
    }

    pub fn draw(&self, ctx: &mut Context) {
        for powerup in &self.powerups {
            match powerup.kind {
                PowerUpKind::AdditionalHeart => self
                    .heart_sprite
                    .draw(ctx, DrawParams::new().position(powerup.position)),
                PowerUpKind::FasterShooting => self
                    .fire_scroll_sprite
                    .draw(ctx, DrawParams::new().position(powerup.position).scale(Vec2::new(2.0, 2.0))),
            }
        }
    }

    pub fn update(&mut self) {
        self.powerups.retain(|powerup| !powerup.is_expired());
    }

    pub fn spawn_power_up(&mut self) {
        self.last_spawned_time = Instant::now();
        let mut rng = StdRng::from_entropy();
        let position = Vec2::new(rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0));

        let power_up = PowerUp {
            kind: rng.gen(),
            spawned_time: self.last_spawned_time,
            position,
        };

        self.powerups.push(power_up);
    }
}
