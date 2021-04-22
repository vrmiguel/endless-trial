use tetra::{Context, graphics::{Color, DrawParams, Texture}, math::Vec2};

use crate::sprites;

pub struct Background {
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
            Vec2::new(775.0, 460.),
        ];

        Self {
            grass_1: Texture::from_file_data(ctx, sprites::GRASS1).unwrap(),
            grass_2: Texture::from_file_data(ctx, sprites::GRASS2).unwrap(),
            rock_1: Texture::from_file_data(ctx, sprites::ROCK1).unwrap(),
            rock_2: Texture::from_file_data(ctx, sprites::ROCK2).unwrap(),
            grass_1_points,
            grass_2_points,
            rock_1_points,
            rock_2_points
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let scale = Vec2::new(1.5, 1.5);
        for point in &self.grass_1_points {
            self.grass_1.draw(ctx, DrawParams::new().position(*point).scale(scale).color(Color::BLUE));
        }

        for point in &self.grass_2_points {
            self.grass_2.draw(ctx, DrawParams::new().position(*point).scale(scale).color(Color::BLUE));
        }

        for point in &self.rock_1_points {
            self.rock_1.draw(ctx, DrawParams::new().position(*point).scale(scale));
        }

        for point in &self.rock_2_points {
            self.rock_2.draw(ctx, DrawParams::new().position(*point).scale(scale));
        }

    }
}
