use std::time::{Duration, Instant};

use arrayvec::ArrayVec;
use rand::{
    distributions::Standard, prelude::Distribution, Rng,
};
use tetra::{
    graphics::{DrawParams, Rectangle},
    math::Vec2,
    Context,
};

use crate::{
    humanoid::Humanoid, panel::Panel, textures::PowerUpTextures,
    timer::Timer,
};

/// New power-ups spawn every 5 seconds
const POWER_UP_SPAWN_INTERVAL: Duration = Duration::from_secs(3);

/// Power-ups, after spawned, are available to be picked up
/// within 10 seconds from spawning
const POWER_UP_AVAILABILITY_INTERVAL: Duration =
    Duration::from_secs(10);

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum PowerUpKind {
    AdditionalHeart,
    FasterShooting,
    FasterRunning,
    TripleShooting,
}

impl From<PowerUpKind> for u8 {
    fn from(kind: PowerUpKind) -> Self {
        match kind {
            PowerUpKind::AdditionalHeart => 0,
            PowerUpKind::FasterShooting => 1,
            PowerUpKind::FasterRunning => 2,
            PowerUpKind::TripleShooting => 3,
        }
    }
}

pub struct ActivePowerUps {
    pub slots: [Option<Instant>; 3],
}

impl ActivePowerUps {
    pub fn new() -> Self {
        Self { slots: [None; 3] }
    }

    pub fn currently_active(&self) -> (bool, bool, bool) {
        (
            self.get(PowerUpKind::FasterShooting).is_some(),
            self.get(PowerUpKind::FasterRunning).is_some(),
            self.get(PowerUpKind::TripleShooting).is_some(),
        )
    }

    /// How many kinds of power-ups are currently active
    pub fn len(&self) -> usize {
        let (a, b, c) = self.currently_active();

        ((a as u8) + (b as u8) + (c as u8)) as usize
    }

    /// An iterator of all active power-ups when called
    pub fn iter(&self) -> impl Iterator<Item = PowerUpKind> {
        let mut power_ups: ArrayVec<PowerUpKind, 3> =
            ArrayVec::new();

        let (fast_shooting, fast_running, triple_shooting) =
            self.currently_active();

        if fast_running {
            power_ups.push(PowerUpKind::FasterRunning)
        }

        if fast_shooting {
            power_ups.push(PowerUpKind::FasterShooting)
        }

        if triple_shooting {
            power_ups.push(PowerUpKind::TripleShooting)
        }

        power_ups.into_iter()
    }

    pub fn activate_power_up(&mut self, kind: PowerUpKind) {
        let idx: u8 = kind.into();

        self.slots[idx as usize - 1] = Some(Instant::now());
    }

    fn get(&self, kind: PowerUpKind) -> Option<Instant> {
        let idx: u8 = kind.into();

        self.slots[idx as usize - 1]
    }
}

#[derive(Debug)]
struct PowerUp {
    /// What sort of power-up this iss
    kind: PowerUpKind,
    /// Times how long this power-up will be available for
    expiration_timer: Timer,
    /// The position the power-up was spawned in
    position: Vec2<f32>,
    /// Whether or not this power-up has been consumed
    was_consumed: bool,
    /// Set if the power-up is flickering (that is, next to
    /// expirating)
    flickering: u8,
}

impl PowerUp {
    pub fn is_expired(&self) -> bool {
        self.expiration_timer.is_ready()
    }

    pub fn flicker_if_almost_expiring(&mut self) {
        // If flickering is already set then do nothing
        if self.flickering > 0 {
            return;
        }

        let elapsed = self.expiration_timer.elapsed();
        if elapsed > Duration::from_secs(8) {
            // Only two more seconds available to get the
            // power-up, so we'll signal this to the
            // player through flickering
            self.flickering = 120;
        }
    }
}

impl Distribution<PowerUpKind> for Standard {
    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
    ) -> PowerUpKind {
        match rng.gen_range(0..=3) {
            0 => PowerUpKind::AdditionalHeart,
            1 => PowerUpKind::FasterShooting,
            2 => PowerUpKind::TripleShooting,
            _ => PowerUpKind::FasterRunning,
        }
    }
}

pub struct PowerUpManager {
    // The power-ups laying on the ground
    powerups: Vec<PowerUp>,
    spawn_timer: Timer,
    panel: Panel,
    power_up_textures: PowerUpTextures,
}

impl PowerUpManager {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            power_up_textures: PowerUpTextures::load(ctx),
            powerups: Vec::with_capacity(5),
            spawn_timer: Timer::start_now_with_interval(
                POWER_UP_SPAWN_INTERVAL,
            ),
            panel: Panel::new(ctx),
        }
    }

    /// Check if the given humanoid collided with a power-up
    /// laying in the ground.
    pub fn check_for_collision(
        &mut self,
        humanoid: &mut Humanoid,
    ) {
        let pos = humanoid.position;
        let rect = Rectangle::new(pos.x, pos.y, 16.0, 16.0);
        for powerup in &mut self.powerups {
            let powerup_rect = Rectangle::new(
                powerup.position.x,
                powerup.position.y,
                32.0,
                32.0,
            );

            if powerup_rect.intersects(&rect) {
                powerup.was_consumed = true;
                match powerup.kind {
                    PowerUpKind::AdditionalHeart => {
                        humanoid.hearts += 1
                    }
                    power_up => {
                        humanoid
                            .power_ups
                            .activate_power_up(power_up);
                    }
                }
            }
        }
    }

    pub fn can_spawn(&self) -> bool {
        self.spawn_timer.is_ready()
    }

    fn draw_powerup_bar(
        &self,
        ctx: &mut Context,
        player_power_ups: &ActivePowerUps,
    ) {
        let active_powerups_no = player_power_ups.len();
        if active_powerups_no == 0 {
            return;
        }

        let width = (active_powerups_no as f32) * 16.0 + 10.5;

        self.panel.sprite.draw_nine_slice(
            ctx,
            &self.panel.config,
            width,
            26.0,
            DrawParams::new()
                .position(Vec2::new(768.0 - width, 60.0)),
        );

        for (kind, spacing) in
            player_power_ups.iter().zip(0..active_powerups_no)
        {
            let spacing = spacing as f32;
            match kind {
                PowerUpKind::AdditionalHeart => unreachable!(),
                PowerUpKind::FasterShooting => self
                    .power_up_textures
                    .fire_scroll_sprite
                    .draw(
                        ctx,
                        DrawParams::new().position(Vec2 {
                            x: 746. - 16.0 * spacing,
                            y: 60. + 4.,
                        }),
                    ),
                PowerUpKind::FasterRunning => {
                    self.power_up_textures.boot_sprite.draw(
                        ctx,
                        DrawParams::new().position(Vec2 {
                            x: 746. - 16.0 * spacing,
                            y: 60. + 4.,
                        }),
                    )
                }
                PowerUpKind::TripleShooting => {
                    self.power_up_textures.ring_sprite.draw(
                        ctx,
                        DrawParams::new().position(Vec2 {
                            x: 746. - 16.0 * spacing,
                            y: 60. + 4.,
                        }),
                    )
                }
            }
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        player_power_ups: &ActivePowerUps,
    ) {
        self.draw_powerup_bar(ctx, player_power_ups);

        for powerup in self.powerups.iter_mut() {
            if powerup.flickering > 0 {
                powerup.flickering -= 1;
                if powerup.flickering % 2 == 0 {
                    continue;
                }
            }

            match powerup.kind {
                PowerUpKind::AdditionalHeart => {
                    self.power_up_textures.heart_sprite.draw(
                        ctx,
                        DrawParams::new()
                            .position(powerup.position),
                    )
                }
                PowerUpKind::FasterShooting => self
                    .power_up_textures
                    .fire_scroll_sprite
                    .draw(
                        ctx,
                        DrawParams::new()
                            .position(powerup.position)
                            .scale(Vec2::new(2.5, 2.5)),
                    ),
                PowerUpKind::FasterRunning => {
                    self.power_up_textures.boot_sprite.draw(
                        ctx,
                        DrawParams::new()
                            .position(powerup.position)
                            .scale(Vec2::new(2.5, 2.5)),
                    )
                }
                PowerUpKind::TripleShooting => {
                    self.power_up_textures.ring_sprite.draw(
                        ctx,
                        DrawParams::new()
                            .position(powerup.position)
                            .scale(Vec2::new(2.5, 2.5)),
                    )
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.powerups
            .retain(|p| !p.was_consumed && !p.is_expired());

        self.powerups
            .iter_mut()
            .for_each(PowerUp::flicker_if_almost_expiring);
    }

    pub fn advance<R: Rng>(
        &mut self,
        rng: &mut R,
        player: &mut Humanoid,
    ) {
        if self.can_spawn() {
            self.spawn_power_up(rng);
        }

        self.check_for_collision(player);
    }

    pub fn spawn_power_up<R: Rng>(&mut self, rng: &mut R) {
        self.spawn_timer.reset();

        let position = Vec2 {
            x: rng.gen_range(0.0..800.0),
            y: rng.gen_range(0.0..800.0),
        };

        let power_up = PowerUp {
            kind: rng.gen(),
            position,
            expiration_timer: Timer::start_now_with_interval(
                POWER_UP_AVAILABILITY_INTERVAL,
            ),
            was_consumed: false,
            flickering: 0,
        };

        self.powerups.push(power_up);
    }
}
