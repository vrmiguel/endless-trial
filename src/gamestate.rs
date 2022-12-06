use std::time::{Duration, Instant};

use graphics::Color;
use rand::{
    prelude::{SliceRandom, StdRng},
    SeedableRng,
};
use tetra::graphics;
use tetra::time;
use tetra::window;
use tetra::{
    graphics::{
        scaling::{ScalingMode, ScreenScaler},
        Texture,
    },
    input::{self, Key},
    math::Vec2,
    Context, Event, State,
};

use crate::{
    animation::FireballAnimation,
    background::Background,
    enemy::EnemyManager,
    healthbar::HealthBar,
    humanoid::{Humanoid, HumanoidType},
    oneoffanim::OneOffAnimationManager,
    panel::GameOverPanel,
    powerup::PowerUpManager,
    projectile::ProjectileManager,
    resources::{self},
    HEIGHT, WIDTH,
};
use crate::{down, left, right, up};

const WAVES: [[(HumanoidType, f32); 4]; 6] = [
    [
        (HumanoidType::BasicEnemy, 0.85),
        (HumanoidType::StrongerEnemy, 0.10),
        (HumanoidType::BadassEnemy, 0.05),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.75),
        (HumanoidType::StrongerEnemy, 0.20),
        (HumanoidType::BadassEnemy, 0.05),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.75),
        (HumanoidType::StrongerEnemy, 0.20),
        (HumanoidType::BadassEnemy, 0.05),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.4),
        (HumanoidType::StrongerEnemy, 0.5),
        (HumanoidType::BadassEnemy, 0.1),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.1),
        (HumanoidType::StrongerEnemy, 0.6),
        (HumanoidType::BadassEnemy, 0.3),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.0),
        (HumanoidType::StrongerEnemy, 0.55),
        (HumanoidType::BadassEnemy, 0.35),
        (HumanoidType::Boss, 0.1),
    ],
];

pub struct GameState {
    scaler: ScreenScaler,
    background: Background,
    health_bar: HealthBar,
    player: Humanoid,
    fireball_mgr: ProjectileManager,
    power_up_mgr: PowerUpManager,
    enemy_mgr: EnemyManager,
    one_off_anim_mgr: OneOffAnimationManager,
    game_over_panel: GameOverPanel,
    game_is_over: bool,
    rng: StdRng,
    player_default_shooting_time: Duration,
    game_score: u64,
    current_wave: u8,
    start_of_this_wave: Instant,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player_texture = Texture::from_encoded(ctx, resources::HERO)?;

        let player = Humanoid::new(
            2,
            player_texture,
            Vec2::new(240.0, 160.0),
            Vec2::new(0.0, 0.0),
            true,
            Duration::from_secs_f32(0.25),
            HumanoidType::Player,
        );
        let background = Background::new(ctx);

        let fireball_animation = FireballAnimation::make_animation(ctx);

        Ok(GameState {
            player,
            background,
            health_bar: HealthBar::new(ctx),
            power_up_mgr: PowerUpManager::new(ctx),
            scaler: ScreenScaler::with_window_size(ctx, WIDTH, HEIGHT, ScalingMode::ShowAll)?,
            fireball_mgr: ProjectileManager::new(fireball_animation),
            game_over_panel: GameOverPanel::new(ctx),
            enemy_mgr: EnemyManager::new(ctx),
            one_off_anim_mgr: OneOffAnimationManager::new(ctx),
            game_is_over: false,
            rng: StdRng::from_entropy(),
            player_default_shooting_time: Duration::from_secs_f32(0.25),
            game_score: 0,
            current_wave: 0,
            start_of_this_wave: Instant::now(),
        })
    }

    fn check_for_wave_change(&mut self) {
        let elapsed = self.start_of_this_wave.elapsed();

        if elapsed > Duration::from_secs(30) && self.current_wave < (WAVES.len() as u8 - 1) {
            self.current_wave += 1;
            self.start_of_this_wave = Instant::now();
            println!("Commencing wave {}", self.current_wave + 1);
        }
    }

    fn check_for_scale_change(&mut self, ctx: &mut Context) {
        if input::is_key_pressed(ctx, Key::F1) {
            let next = match self.scaler.mode() {
                ScalingMode::Fixed => ScalingMode::Stretch,
                ScalingMode::Stretch => ScalingMode::ShowAll,
                ScalingMode::ShowAll => ScalingMode::ShowAllPixelPerfect,
                ScalingMode::ShowAllPixelPerfect => ScalingMode::Crop,
                ScalingMode::Crop => ScalingMode::CropPixelPerfect,
                ScalingMode::CropPixelPerfect => ScalingMode::Fixed,
                _ => ScalingMode::Fixed,
            };

            println!("[LOG] Scaling mode changed to {:?}", next);

            self.scaler.set_mode(next);
        }
    }

    fn triple_shoot(&mut self, angle: f32) {
        self.fireball_mgr.add_projectile(
            angle - 45.0,
            self.player.position,
            Vec2 { x: 6.0, y: 6.0 },
        );
        self.fireball_mgr
            .add_projectile(angle, self.player.position, Vec2 { x: 6.0, y: 6.0 });
        self.fireball_mgr.add_projectile(
            angle + 45.0,
            self.player.position,
            Vec2 { x: 6.0, y: 6.0 },
        );
    }

    // TODO: there's probably a nicer solution to this with algebra
    pub fn check_for_fire(ctx: &mut Context) -> Option<f32> {
        match (left!(ctx), right!(ctx), up!(ctx), down!(ctx)) {
            // These first cases are kind of nonsensical so I'm going to explicitly ignore them
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
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::rgb(118.0 / 255.0, 197.0 / 255.0, 100.0 / 255.0));

        self.background.draw(ctx);

        self.player.advance_animation(ctx);
        self.fireball_mgr.advance_animation(ctx);

        self.player.draw(ctx);
        self.fireball_mgr.draw(ctx);
        self.enemy_mgr.draw(ctx);
        self.power_up_mgr.draw(ctx);
        self.health_bar.draw(ctx, self.player.hearts);
        self.one_off_anim_mgr.draw(ctx);

        if self.game_is_over {
            self.game_over_panel.draw(ctx);
        }

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);

        window::set_title(
            ctx,
            &format!(
                "Endless Trial - {:.0} FPS - Wave: {} - Score: {}",
                time::get_fps(ctx),
                self.current_wave + 1,
                self.game_score
            ),
        );

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Checks if the player changed the screen scaling method
        self.check_for_scale_change(ctx);

        // Freeze the game logic if the game is over
        if self.game_is_over {
            return Ok(());
        }

        if self.player.is_dead() {
            self.game_is_over = true;
            return Ok(());
        }

        // Checks if the current wave is over
        self.check_for_wave_change();

        // Check if the player collided with an enemy
        // We return enemy_rects here (Vec of Retangles for each enemy) in order to reuse it in .check_for_fireball_collisions
        let (collided_with_an_enemy, enemy_rects) = self
            .player
            .collided_with_bodies(self.enemy_mgr.enemies_ref());

        if collided_with_an_enemy {
            self.player.take_hit();
        }

        // Check if an enemy was hit with a fireball from the player
        self.enemy_mgr.check_for_fireball_collisions(
            &enemy_rects,
            self.fireball_mgr.projectiles_ref(),
            &mut self.one_off_anim_mgr,
        );

        // Check if the player was hit with a cannonball from an enemy
        self.enemy_mgr
            .check_for_cannonball_collisions(&mut self.player, &mut self.one_off_anim_mgr);

        if self.power_up_mgr.faster_shooting_active() {
            self.player
                .set_shooting_wait_time(Duration::from_secs_f32(0.08))
        } else {
            self.player
                .set_shooting_wait_time(self.player_default_shooting_time);
        }

        if self.player.can_fire() {
            if let Some(angle) = Self::check_for_fire(ctx) {
                match self.power_up_mgr.triple_shooting_active() {
                    true => self.triple_shoot(angle),
                    false => self.fireball_mgr.add_projectile(
                        angle,
                        self.player.position,
                        Vec2 { x: 5.0, y: 5.0 },
                    ),
                }
                self.player.register_fire();
            }
        }

        if self.enemy_mgr.can_spawn() {
            let kind = WAVES[self.current_wave as usize]
                .choose_weighted(&mut self.rng, |x| x.1)
                .expect("WAVES should not be empty")
                .0;
            self.enemy_mgr.spawn_enemy(ctx, kind, &mut self.rng);
        }

        let enemy_score = self.enemy_mgr.calc_score();

        if self.power_up_mgr.can_spawn() {
            self.power_up_mgr.spawn_power_up();
        }

        self.power_up_mgr.check_for_collision(&mut self.player);

        let hero_speed = if self.power_up_mgr.faster_running_active() {
            4.5
        } else {
            2.1
        };

        // Checks for WASD presses and updates player location
        self.player.update_from_key_press(ctx, hero_speed);

        self.one_off_anim_mgr.update();

        self.enemy_mgr.update(ctx, self.player.position);

        self.power_up_mgr.update();

        self.game_score += enemy_score - self.enemy_mgr.calc_score();

        Ok(())
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }
        Ok(())
    }
}
