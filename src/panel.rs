use tetra::{
    graphics::{
        text::{Font, Text},
        NineSlice, Rectangle, Texture,
    },
    math::Vec2,
    Context,
};

use crate::resources;

pub struct Panel {
    pub sprite: Texture,
    pub config: NineSlice,
}

impl Panel {
    pub fn new(ctx: &mut Context) -> Self {
        let sprite = Texture::from_encoded(ctx, resources::PANEL)
            .expect("failed to load built-in panel sprite");

        Self {
            sprite,
            config: NineSlice::with_border(Rectangle::new(0.0, 0.0, 32.0, 32.0), 4.0),
        }
    }
}

pub struct GameOverPanel {
    panel: Panel,
    text: Text,
}

impl GameOverPanel {
    pub fn new(ctx: &mut Context) -> Self {
        let panel = Panel::new(ctx);
        let text = Text::new(
            "Game over!",
            Font::from_vector_file_data(ctx, resources::BITPOTION_FONT, 64.0)
                .expect("Failed to instantiate font"),
        );

        Self { panel, text }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.panel.sprite.draw_nine_slice(
            ctx,
            &self.panel.config,
            206.0,
            80.,
            Vec2 { x: 320.0, y: 320.0 },
        );

        self.text.draw(
            ctx,
            Vec2 {
                x: 320.0 + 8.0,
                y: 320.0 + 8.0,
            },
        );
    }
}
