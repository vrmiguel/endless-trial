use std::time::{Duration, Instant};

use tetra::{graphics::Texture, math::Vec2, Context};

use crate::{
    animation::FireballAnimation,
    down,
    humanoid::{Humanoid, HumanoidType},
    left,
    projectile::{Projectile, ProjectileManager},
    resources, right,
    traits::Cleanable,
    up,
};

pub struct PlayerManager {
    player: Humanoid,
    fireball_mgr: ProjectileManager,
}

impl PlayerManager {
    pub fn is_player_dead(&self) -> bool {
        self.player.is_dead()
    }

    pub fn hearts(&self) -> u8 {
        self.player.hearts
    }

    pub fn player_position(&self) -> Vec2<f32> {
        self.player.position
    }

    pub fn register_hit(&mut self) {
        self.player.take_hit()
    }

    pub fn player_mut(&mut self) -> &mut Humanoid {
        &mut self.player
    }

    pub fn fireballs(&self) -> &[Projectile] {
        self.fireball_mgr.projectiles()
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.player.clean_up();

        let (
            faster_shooting_active,
            triple_shooting_active,
            faster_running_active,
        ) = self.player.power_ups.currently_active();

        let wait_time = if faster_shooting_active {
            Duration::from_secs_f32(0.08)
        } else {
            Duration::from_secs_f32(0.25)
        };

        self.player
            .shooting_behavior
            .set_shooting_wait_time(wait_time);

        if self.player.can_fire() {
            if let Some(angle) = Self::check_for_fire(ctx) {
                self.fireball_mgr.shoot(
                    triple_shooting_active,
                    angle,
                    self.player.position,
                    Vec2 { x: 5.5, y: 5.5 },
                );

                self.player.shooting_behavior.register_fire();
            }
        }

        let hero_speed =
            if faster_running_active { 4.5 } else { 2.1 };

        // Checks for WASD presses and updates player location
        self.player.update_from_key_press(ctx, hero_speed);
    }

    pub fn new(ctx: &mut Context) -> Self {
        let now = Instant::now();

        let player_texture =
            Texture::from_encoded(ctx, resources::HERO).unwrap();

        let fireball_animation = FireballAnimation::build(ctx);

        let player_mgr = Self {
            player: Humanoid::new(
                2,
                player_texture,
                Vec2::new(240.0, 160.0),
                Vec2::new(0.0, 0.0),
                true,
                Duration::from_secs_f32(0.25),
                HumanoidType::Player,
            ),
            fireball_mgr: ProjectileManager::new(
                fireball_animation,
            ),
        };

        println!(
            "Built PlayerManager in {}ms",
            now.elapsed().as_millis()
        );

        player_mgr
    }

    // TODO: there's probably a nicer solution to this with
    // algebra
    pub fn check_for_fire(ctx: &mut Context) -> Option<f32> {
        match (left!(ctx), right!(ctx), up!(ctx), down!(ctx)) {
            // These first cases are kind of nonsensical so I'm
            // going to explicitly ignore them
            (true, true, _, _) => None,
            (_, _, true, true) => None,
            (true, false, true, false) => {
                // Left and Up -> 135 deg
                Some(135.0)
            }
            (true, false, false, true) => {
                // Left and Down -> 225 deg
                Some(225.0)
            }
            (false, true, false, true) => {
                // Right and Down -> 315 deg
                Some(315.0)
            }
            (false, true, true, false) => {
                // Right and Up -> 45 deg
                Some(45.0)
            }
            (true, false, false, false) => {
                // Only Left -> 180 deg
                Some(180.0)
            }
            (false, true, false, false) => {
                // Only Right -> 0 deg
                Some(0.0)
            }
            (false, false, true, false) => {
                // Only Up -> 90 deg
                Some(90.0)
            }
            (false, false, false, true) => {
                // Only Down -> 270 deg
                Some(270.0)
            }
            (false, false, false, false) => {
                // No arrow buttons pressed
                None
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.player.advance_animation(ctx);
        self.fireball_mgr.advance_animation(ctx);

        self.player.draw(ctx);
        self.fireball_mgr.draw(ctx);
    }
}
