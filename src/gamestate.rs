use graphics::Color;
use tetra::{Context, Event, State, graphics::{Texture, scaling::{ScalingMode, ScreenScaler}}, input::{self, Key}, math::Vec2};
use tetra::graphics;
use tetra::window;
use tetra::time;

use crate::{HEIGHT, WIDTH, background::Background, fireball::FireballManager, humanoid::Humanoid, sprites};
use crate::{left, right, up, down};

pub struct GameState {
    scaler: ScreenScaler,
    background: Background,
    player: Humanoid,
    grunt: Humanoid,
    fireball_mgr: FireballManager
}


impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        let player_texture = Texture::from_file_data(ctx, sprites::HERO)?;
        let grunt_texture = Texture::from_file_data(ctx, sprites::HERO_INVINCIBLE)?;

        let player = Humanoid::new(player_texture, Vec2::new(240.0, 160.0), Vec2::new(0.0, 0.0));
        let grunt = Humanoid::new(grunt_texture, Vec2::new(120.0, 120.0), Vec2::new(0.0, 0.0));
        let background = Background::new(ctx);

        Ok(GameState {
            player,
            grunt,
            background,
            scaler: ScreenScaler::with_window_size(
                ctx,
                WIDTH,
                HEIGHT,
                ScalingMode::ShowAllPixelPerfect,
            )?,
            fireball_mgr: FireballManager::new(ctx)
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

    pub fn check_for_fire(ctx: &mut Context) -> Option<f32> {
        match (left!(ctx), right!(ctx), up!(ctx), down!(ctx)) {
            // These first cases are kind of nonsensical so I'm going to explicitly ignore them
            (true, true, _, _) => None,
            (_, _, true, true) => None,
            (true, false, true, false) => {
                // Left and Up -> 135 deg
                Some(135.0)
            },
            (true, false, false, true) => {
                // Left and Down -> 225 deg
                Some(225.0)
            },
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
        graphics::clear(ctx, Color::rgb(118.0/255.0, 197.0/255.0, 100.0/255.0));

        self.background.draw(ctx);
        self.player.advance_animation(ctx);
        self.grunt.advance_animation(ctx);
        self.player.draw(ctx);

        self.grunt.draw(ctx);

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);

        window::set_title(ctx, &format!("joguinho - {:.0} FPS", time::get_fps(ctx)));

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.check_for_scale_change(ctx);
        let _dir = Self::check_for_fire(ctx);

        self.player.update(ctx);
        self.grunt.update(ctx);

        Ok(())
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }
        Ok(())
    }
}