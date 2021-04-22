mod animation;
mod background;
mod direction;
mod humanoid;
mod sprites;

use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::time;
use tetra::window;
use tetra::{
    graphics::scaling::{ScalingMode, ScreenScaler},
    input::{self, Key},
    Event,
};
use tetra::{Context, ContextBuilder, State};

use direction::Direction;
use humanoid::Humanoid;
use background::Background;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

struct GameState {
    scaler: ScreenScaler,
    background: Background,
    player: Humanoid,
    grunt: Humanoid,
}

const VERSION: &str = "0.1.0";

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

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
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::rgb(89.0/255.0, 187.0/255.0, 117.0/255.0));

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
    println!("joguinho v{}", VERSION);
    ContextBuilder::new("my lil game", WIDTH, HEIGHT)
        .quit_on_escape(true)
        .debug_info(true)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
