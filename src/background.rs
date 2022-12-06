use tetra::{
    graphics::{Color, DrawParams, Texture},
    math::Vec2,
    Context,
};

use crate::resources;

pub struct Background {
    grass_left: Texture,
    grass_lower_right: Texture,
    grass_lower_left: Texture,
    grass_lower: Texture,
    grass_top_left: Texture,
    grass_top: Texture,
    grass_top_right: Texture,
    grass_right: Texture,

    grass_1: Texture,
    grass_2: Texture,
    rock_1: Texture,
    rock_2: Texture,
    rock_1_points: Vec<Vec2<f32>>,
    rock_2_points: Vec<Vec2<f32>>,
    grass_1_points: Vec<Vec2<f32>>,
    grass_2_points: Vec<Vec2<f32>>,
}

impl Background {
    pub fn new(ctx: &mut Context) -> Self {
        let rock_1_points = vec![
            Vec2::new(50.0, 60.),
            Vec2::new(150.0, 260.),
            Vec2::new(350.0, 260.),
            Vec2::new(50.0, 660.),
        ];

        let grass_1_points = vec![
            Vec2::new(150.0, 160.),
            Vec2::new(220.0, 120.),
            Vec2::new(290.0, 520.),
            Vec2::new(720.0, 730.),
            Vec2::new(750.0, 260.),
            Vec2::new(675.0, 360.),
        ];

        let grass_2_points = vec![
            Vec2::new(320.0, 520.),
            Vec2::new(520.0, 530.),
            Vec2::new(150.0, 260.),
            Vec2::new(575.0, 660.),
        ];

        let rock_2_points = vec![
            Vec2::new(120.0, 502.),
            Vec2::new(320.0, 132.),
            Vec2::new(650.0, 660.),
        ];

        Self {
            grass_left: Texture::from_encoded(ctx, resources::GRASS_LEFT).unwrap(),
            grass_lower: Texture::from_encoded(ctx, resources::GRASS_LOWER).unwrap(),
            grass_lower_left: Texture::from_encoded(ctx, resources::GRASS_LOWER_LEFT).unwrap(),
            grass_lower_right: Texture::from_encoded(ctx, resources::GRASS_LOWER_RIGHT).unwrap(),
            grass_top_left: Texture::from_encoded(ctx, resources::GRASS_TOP_LEFT).unwrap(),
            grass_top: Texture::from_encoded(ctx, resources::GRASS_TOP).unwrap(),
            grass_top_right: Texture::from_encoded(ctx, resources::GRASS_TOP_RIGHT).unwrap(),
            grass_right: Texture::from_encoded(ctx, resources::GRASS_RIGHT).unwrap(),
            grass_1: Texture::from_encoded(ctx, resources::GRASS_DETAIL_1).unwrap(),
            grass_2: Texture::from_encoded(ctx, resources::GRASS_DETAIL_2).unwrap(),
            rock_1: Texture::from_encoded(ctx, resources::ROCK1).unwrap(),
            rock_2: Texture::from_encoded(ctx, resources::ROCK2).unwrap(),
            grass_1_points,
            grass_2_points,
            rock_1_points,
            rock_2_points,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let scale = Vec2::new(1.5, 1.5);

        for x in (32..725).step_by(75) {
            self.grass_lower
                .draw(ctx, DrawParams::new().position(Vec2::new(x as f32, 768.0)));
        }

        for y in (32..768).step_by(32) {
            self.grass_left
                .draw(ctx, DrawParams::new().position(Vec2::new(0.0, y as f32)));
        }

        for y in (32..768).step_by(32) {
            self.grass_right
                .draw(ctx, DrawParams::new().position(Vec2::new(768., y as f32)));
        }

        for x in (32..770).step_by(70) {
            self.grass_top
                .draw(ctx, DrawParams::new().position(Vec2::new(x as f32, 0.0)));
        }

        self.grass_lower_left
            .draw(ctx, DrawParams::new().position(Vec2::new(0.0, 768.0)));
        self.grass_lower_right
            .draw(ctx, DrawParams::new().position(Vec2::new(768.0, 768.0)));
        self.grass_top_left
            .draw(ctx, DrawParams::new().position(Vec2::new(0., 0.)));
        self.grass_top_right
            .draw(ctx, DrawParams::new().position(Vec2::new(768., 0.0)));

        for point in &self.grass_1_points {
            self.grass_1.draw(
                ctx,
                DrawParams::new()
                    .position(*point)
                    .scale(scale)
                    .color(Color::BLUE),
            );
        }

        for point in &self.grass_2_points {
            self.grass_2.draw(
                ctx,
                DrawParams::new()
                    .position(*point)
                    .scale(scale)
                    .color(Color::BLUE),
            );
        }

        for point in &self.rock_1_points {
            self.rock_1
                .draw(ctx, DrawParams::new().position(*point).scale(scale));
        }

        for point in &self.rock_2_points {
            self.rock_2
                .draw(ctx, DrawParams::new().position(*point).scale(scale));
        }
    }
}
