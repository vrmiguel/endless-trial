use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::{
    distributions::Standard,
    prelude::{Distribution, StdRng},
    Rng, SeedableRng,
};
use tetra::{
    graphics::{DrawParams, Rectangle, Texture},
    math::Vec2,
    Context,
};

use crate::{humanoid::Humanoid, panel::Panel, resources};

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum PowerUpKind {
    AdditionalHeart,
    FasterShooting,
    FasterRunning,
    TripleShooting,
}

#[derive(Debug)]
struct PowerUp {
    kind: PowerUpKind,
    spawned_time: Instant,
    position: Vec2<f32>,
    was_consumed: bool,
    flickering: u8,
}

impl PowerUp {
    pub fn is_expired(&self) -> bool {
        let elapsed = self.spawned_time.elapsed();

        elapsed > Duration::from_secs_f32(10.0)
    }

    pub fn flicker_if_almost_expiring(&mut self) {
        if self.flickering > 0 {
            return;
        }
        let elapsed = self.spawned_time.elapsed();
        if elapsed > Duration::from_secs_f32(8.0) {
            self.flickering = 120;
        }
    }
}

impl Distribution<PowerUpKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUpKind {
        match rng.gen_range(0..=3) {
            0 => PowerUpKind::AdditionalHeart,
            1 => PowerUpKind::FasterShooting,
            2 => PowerUpKind::TripleShooting,
            _ => PowerUpKind::FasterRunning,
        }
    }
}

pub struct PowerUpManager {
    fire_scroll_sprite: Texture,
    heart_sprite: Texture,
    boot_sprite: Texture,
    ring_sprite: Texture,
    // The power-ups laying on the ground
    powerups: Vec<PowerUp>,
    // Timed power-ups consumed
    active_powerups: HashMap<PowerUpKind, Instant>,
    last_spawned_time: Instant,
    panel: Panel,
}

impl PowerUpManager {
    pub fn new(ctx: &mut Context) -> Self {
        let fire_scroll_sprite = Texture::from_encoded(ctx, resources::FIRE_SCROLL)
            .expect("failed to load built-in strawberry sprite");
        let heart_sprite = Texture::from_encoded(ctx, resources::HEART_32X)
            .expect("failed to load built-in heart 32x32 sprite");
        let boot_sprite = Texture::from_encoded(ctx, resources::BOOT)
            .expect("failed to load built-in boot sprite");
        let ring_sprite = Texture::from_encoded(ctx, resources::RING)
            .expect("failed to load built-in ring sprite");

        Self {
            fire_scroll_sprite,
            heart_sprite,
            boot_sprite,
            ring_sprite,
            powerups: vec![],
            active_powerups: HashMap::new(),
            last_spawned_time: Instant::now(),
            panel: Panel::new(ctx),
        }
    }

    pub fn check_for_collision(&mut self, player: &mut Humanoid) {
        let player_pos = player.position;
        let player_rect = Rectangle::new(player_pos.x, player_pos.y, 16.0, 16.0);
        for powerup in &mut self.powerups {
            let powerup_rect = Rectangle::new(powerup.position.x, powerup.position.y, 32.0, 32.0);

            if powerup_rect.intersects(&player_rect) {
                powerup.was_consumed = true;
                match powerup.kind {
                    PowerUpKind::AdditionalHeart => player.hearts += 1,
                    p => {
                        self.active_powerups.insert(p, Instant::now());
                    }
                }
            }
        }
    }

    pub fn faster_shooting_active(&self) -> bool {
        self.active_powerups
            .get(&PowerUpKind::FasterShooting)
            .is_some()
    }

    pub fn faster_running_active(&self) -> bool {
        self.active_powerups
            .get(&PowerUpKind::FasterRunning)
            .is_some()
    }

    pub fn triple_shooting_active(&self) -> bool {
        self.active_powerups
            .get(&PowerUpKind::TripleShooting)
            .is_some()
    }

    pub fn can_spawn(&self) -> bool {
        let time_since_last_throw = self.last_spawned_time.elapsed();
        time_since_last_throw > Duration::from_secs_f64(5.00)
    }

    fn draw_powerup_bar(&self, ctx: &mut Context) {
        let active_powerups_no = self.active_powerups.len();
        if active_powerups_no == 0 {
            return;
        }

        let width = (active_powerups_no as f32) * 16.0 + 10.5;

        self.panel.sprite.draw_nine_slice(
            ctx,
            &self.panel.config,
            width,
            26.0,
            DrawParams::new().position(Vec2::new(768.0 - width, 60.0)),
        );

        for (kind, spacing) in self.active_powerups.keys().zip(0..active_powerups_no) {
            let spacing = spacing as f32;
            match kind {
                PowerUpKind::AdditionalHeart => unreachable!(),
                PowerUpKind::FasterShooting => self.fire_scroll_sprite.draw(
                    ctx,
                    DrawParams::new().position(Vec2 {
                        x: 746. - 16.0 * spacing,
                        y: 60. + 4.,
                    }),
                ),
                PowerUpKind::FasterRunning => self.boot_sprite.draw(
                    ctx,
                    DrawParams::new().position(Vec2 {
                        x: 746. - 16.0 * spacing,
                        y: 60. + 4.,
                    }),
                ),
                PowerUpKind::TripleShooting => self.ring_sprite.draw(
                    ctx,
                    DrawParams::new().position(Vec2 {
                        x: 746. - 16.0 * spacing,
                        y: 60. + 4.,
                    }),
                ),
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.draw_powerup_bar(ctx);

        for powerup in self.powerups.iter_mut() {
            if powerup.flickering > 0 {
                powerup.flickering -= 1;
                if powerup.flickering % 2 == 0 {
                    continue;
                }
            }

            match powerup.kind {
                PowerUpKind::AdditionalHeart => self
                    .heart_sprite
                    .draw(ctx, DrawParams::new().position(powerup.position)),
                PowerUpKind::FasterShooting => self.fire_scroll_sprite.draw(
                    ctx,
                    DrawParams::new()
                        .position(powerup.position)
                        .scale(Vec2::new(2.5, 2.5)),
                ),
                PowerUpKind::FasterRunning => self.boot_sprite.draw(
                    ctx,
                    DrawParams::new()
                        .position(powerup.position)
                        .scale(Vec2::new(2.5, 2.5)),
                ),
                PowerUpKind::TripleShooting => self.ring_sprite.draw(
                    ctx,
                    DrawParams::new()
                        .position(powerup.position)
                        .scale(Vec2::new(2.5, 2.5)),
                ),
            }
        }
    }

    pub fn update(&mut self) {
        self.active_powerups
            .retain(|_, instant| instant.elapsed() < Duration::from_secs_f32(5.0));

        self.powerups
            .iter_mut()
            .for_each(|p| p.flicker_if_almost_expiring());
        self.powerups.retain(|p| !p.was_consumed && !p.is_expired());
    }

    pub fn spawn_power_up(&mut self) {
        self.last_spawned_time = Instant::now();
        let mut rng = StdRng::from_entropy();
        let position = Vec2 {
            x: rng.gen_range(0.0..800.0),
            y: rng.gen_range(0.0..800.0),
        };

        let power_up = PowerUp {
            kind: rng.gen(),
            spawned_time: self.last_spawned_time,
            position,
            was_consumed: false,
            flickering: 0,
        };

        self.powerups.push(power_up);
    }
}
