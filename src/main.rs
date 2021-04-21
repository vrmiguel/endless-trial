mod animation;
mod direction;
mod humanoid;
mod sprites;

use std::time::Duration;

use tetra::{graphics::animation::Animation, input::{self, Key}};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

use direction::Direction;
use humanoid::Humanoid;

struct GameState {
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
            grunt
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        
        self.player.advance_animation(ctx);
        self.grunt.advance_animation(ctx);

        graphics::clear(ctx, Color::rgb(0.294, 0.61, 0.16));

        self.player.draw(ctx);
        self.grunt.draw(ctx);


        Ok(())
    }


    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.player.update(ctx);
        self.grunt.update(ctx);

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("my lil game", 480, 320)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
