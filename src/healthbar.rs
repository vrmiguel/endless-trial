use tetra::{Context, graphics::{DrawParams, NineSlice, Rectangle, Texture}, math::Vec2};

use crate::sprites;

// A small utility struct to draw hearts on the screen
pub struct HealthBar {
    heart_sprite: Texture,
    panel_sprite: Texture,
    panel_config: NineSlice,
}

impl HealthBar {
    pub fn new(ctx: &mut Context) -> Self {
        let heart_sprite = Texture::from_file_data(ctx, sprites::HEART_16X)
            .expect("could not load built-in heart sprite");

        let panel_sprite = Texture::from_file_data(ctx, sprites::PANEL)
            .expect("failed to load built-in panel sprite");

        Self {
            panel_sprite,
            panel_config: NineSlice::with_border(Rectangle::new(0.0, 0.0, 32.0, 32.0), 4.0),
            heart_sprite,
        }
    }

    pub fn draw(&self, ctx: &mut Context, number_of_hearts: u8) {
        
        let width = (number_of_hearts as f32) * 16.0 + 10.5;
        self.panel_sprite.draw_nine_slice(
            ctx, 
            &self.panel_config, 
            width,
            26.0, 
            DrawParams::new()
                    .position(Vec2::new(768.0 - width, 32.0))
            );
        
        for spacing in 0..number_of_hearts {
            let spacing = spacing as f32;
            self.heart_sprite
                .draw(ctx, DrawParams::new().position(Vec2::new(746.0 - 16.0 * spacing, 36.0)));
        }
        

        // self.heart_sprite
        //     .draw(ctx, DrawParams::new().position(Vec2::new(746.0 - 16.0, 36.0)))
    }
}
