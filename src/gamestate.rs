use graphics::Color;
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
    background::Background,
    enemy::EnemyManager,
    fireball::FireballManager,
    healthbar::HealthBar,
    humanoid::{Humanoid, HumanoidType},
    panel::GameOverPanel,
    powerup::PowerUpManager,
    resources, HEIGHT, WIDTH,
};
use crate::{down, left, right, up};

pub struct GameState {
    scaler: ScreenScaler,
    background: Background,
    health_bar: HealthBar,
    player: Humanoid,
    fireball_mgr: FireballManager,
    power_up_mgr: PowerUpManager,
    enemy_mgr: EnemyManager,
    game_over_panel: GameOverPanel,
    game_is_over: bool,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player_texture = Texture::from_file_data(ctx, resources::HERO)?;

        let player = Humanoid::new(
            player_texture,
            Vec2::new(240.0, 160.0),
            Vec2::new(0.0, 0.0),
            HumanoidType::Player,
        );
        let background = Background::new(ctx);

        Ok(GameState {
            player,
            background,
            health_bar: HealthBar::new(ctx),
            power_up_mgr: PowerUpManager::new(ctx),
            scaler: ScreenScaler::with_window_size(ctx, WIDTH, HEIGHT, ScalingMode::ShowAll)?,
            fireball_mgr: FireballManager::new(ctx),
            game_over_panel: GameOverPanel::new(ctx),
            enemy_mgr: EnemyManager::new(),
            game_is_over: false,
        })
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
            };

            println!("[LOG] Scaling mode changed to {:?}", next);

            self.scaler.set_mode(next);
        }
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
        self.health_bar.draw(ctx, self.player.health());

        if self.game_is_over {
            self.game_over_panel.draw(ctx);
        }

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);

        window::set_title(ctx, &format!("joguinho - {:.0} FPS", time::get_fps(ctx)));

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.game_is_over {
            return Ok(());
        }

        self.check_for_scale_change(ctx);

        let (collided_with_an_enemy, enemy_rects) = self
            .player
            .collided_with_bodies(self.enemy_mgr.enemies_ref());

        if collided_with_an_enemy {
            if self.player.flickering == 0 {
                self.player.hearts -= 1;
                if self.player.hearts == 0 {
                    self.game_is_over = true;
                }
                self.player.flickering = 30;
            }
        }

        self.enemy_mgr
            .check_for_fireball_collisions(&enemy_rects, self.fireball_mgr.fireballs_ref());

        if self.fireball_mgr.can_throw() {
            match Self::check_for_fire(ctx) {
                Some(angle) => self
                    .fireball_mgr
                    .add_fireball(angle, self.player.get_position()),
                None => {}
            }
        }

        if self.enemy_mgr.can_spawn() {
            self.enemy_mgr.spawn_enemy(ctx, HumanoidType::BasicEnemy);
        }

        if self.power_up_mgr.can_spawn() {
            self.power_up_mgr.spawn_power_up();
        }

        self.power_up_mgr.check_for_collision(&mut self.player);

        // Checks for WASD presses and updates player location
        self.player.update_from_key_press(ctx);

        self.enemy_mgr.update(ctx, self.player.get_position());

        self.power_up_mgr.update();

        Ok(())
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }
        Ok(())
    }
}
