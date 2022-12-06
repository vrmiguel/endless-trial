use tetra::{
    graphics::{DrawParams, Texture},
    math::Vec2,
    Context,
};

use crate::panel::Panel;
use crate::resources;

// A small utility struct to draw hearts on the screen
pub struct HealthBar {
    panel: Panel,
    heart_sprite: Texture,
}

impl HealthBar {
    pub fn new(ctx: &mut Context) -> Self {
        let heart_sprite = Texture::from_encoded(ctx, resources::HEART_16X)
            .expect("could not load built-in heart sprite");

        Self {
            panel: Panel::new(ctx),
            heart_sprite,
        }
    }

    pub fn draw(&self, ctx: &mut Context, number_of_hearts: u8) {
        let width = (number_of_hearts as f32) * 16.0 + 10.5;
        self.panel.sprite.draw_nine_slice(
            ctx,
            &self.panel.config,
            width,
            26.0,
            DrawParams::new().position(Vec2::new(768.0 - width, 32.0)),
        );

        for spacing in 0..number_of_hearts {
            let spacing = spacing as f32;
            self.heart_sprite.draw(
                ctx,
                DrawParams::new().position(Vec2::new(746.0 - 16.0 * spacing, 36.0)),
            );
        }

        // self.heart_sprite
        //     .draw(ctx, DrawParams::new().position(Vec2::new(746.0 - 16.0, 36.0)))
    }
}
