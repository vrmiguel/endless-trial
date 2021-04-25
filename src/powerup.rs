use std::time::{Duration, Instant};

use rand::{
    distributions::Standard,
    prelude::{Distribution, StdRng},
    Rng, SeedableRng,
};
use tetra::{Context, graphics::{DrawParams, Rectangle, Texture}, math::Vec2};

use crate::{humanoid::Humanoid, sprites};

pub enum PowerUpKind {
    AdditionalHeart,
    FasterShooting,
}
struct PowerUp {
    kind: PowerUpKind,
    spawned_time: Instant,
    position: Vec2<f32>,
    was_consumed: bool,
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

    pub fn check_for_collision(&mut self, player: &mut Humanoid) {
        let player_pos = player.get_position();
        let player_rect = Rectangle::new(player_pos.x, player_pos.y, 16.0, 16.0);
        for powerup in &mut self.powerups {
            let powerup_rect = Rectangle::new(
                powerup.position.x, 
                powerup.position.y,
                // TODO: not sure if 16 is the right width/height here   
                32.0, 
                32.0);
            
            if powerup_rect.intersects(&player_rect) {
                powerup.was_consumed = true;
            }
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
        self.powerups.retain(|p| !p.was_consumed && !p.is_expired());
    }

    pub fn spawn_power_up(&mut self) {
        self.last_spawned_time = Instant::now();
        let mut rng = StdRng::from_entropy();
        let position = Vec2 { x: rng.gen_range(0.0..800.0), y: rng.gen_range(0.0..800.0)};

        let power_up = PowerUp {
            kind: rng.gen(),
            spawned_time: self.last_spawned_time,
            position,
            was_consumed: false,
        };

        self.powerups.push(power_up);
    }
}
