mod animation;
mod direction;
mod humanoid;
mod sprites;

use std::time::Duration;

use tetra::{Event, graphics::{animation::Animation, scaling::{ScalingMode, ScreenScaler}}, input::{self, Key}};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

use direction::Direction;
use humanoid::Humanoid;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 640.0;

struct GameState {
    scaler: ScreenScaler,
    player: Humanoid,
    grunt: Humanoid
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player_texture = Texture::from_file_data(ctx, sprites::HERO)?;
        let grunt_texture = Texture::from_file_data(ctx, sprites::HERO_INVINCIBLE)?;

        let player = Humanoid::new(player_texture, Vec2::new(240.0, 160.0), Vec2::new(0.0, 0.0));
        let grunt = Humanoid::new(grunt_texture, Vec2::new(120.0, 120.0), Vec2::new(0.0, 0.0));

        Ok(GameState {
            player,
            grunt,
            scaler: ScreenScaler::with_window_size(ctx, 640, 640, ScalingMode::ShowAllPixelPerfect)?,
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

            self.scaler.set_mode(next);
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::rgb(0.294, 0.61, 0.16));

        self.player.advance_animation(ctx);
        self.grunt.advance_animation(ctx);
        self.player.draw(ctx);
        self.grunt.draw(ctx);

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        
        self.check_for_scale_change(ctx);

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

fn main() -> tetra::Result {
    ContextBuilder::new("my lil game", 640, 640)
        .quit_on_escape(true)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
