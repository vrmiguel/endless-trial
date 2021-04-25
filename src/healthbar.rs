use tetra::{
    graphics::{DrawParams, Texture},
    math::Vec2,
    Context,
};

use crate::sprites;

// A small utility struct to draw hearts on the screen
pub struct HealthBar {
    health_sprite: Texture,
}

impl HealthBar {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            health_sprite: Texture::from_file_data(ctx, sprites::HEART_16X)
                .expect("could not load built-in heart sprite"),
        }
    }

    pub fn draw(&self, ctx: &mut Context, _number_of_hearts: u8) {
        self.health_sprite
            .draw(ctx, DrawParams::new().position(Vec2::new(750.0, 50.0)))
    }
}
