use tetra::{
    graphics::{self, Color, DrawParams, Texture},
    math::Vec2,
    Context,
};

use crate::resources;

pub struct Background {
    /// Actual grass background textures
    grass: Grass,
    grass_1: Texture,
    grass_2: Texture,
    rock_1: Texture,
    rock_2: Texture,
}

struct Grass {
    left: Texture,
    lower_right: Texture,
    lower_left: Texture,
    lower: Texture,
    top_left: Texture,
    top: Texture,
    top_right: Texture,
    right: Texture,
}

impl Grass {
    pub fn load(ctx: &mut Context) -> Self {
        Self {
            left: Texture::from_encoded(
                ctx,
                resources::GRASS_LEFT,
            )
            .unwrap(),
            lower: Texture::from_encoded(
                ctx,
                resources::GRASS_LOWER,
            )
            .unwrap(),
            lower_left: Texture::from_encoded(
                ctx,
                resources::GRASS_LOWER_LEFT,
            )
            .unwrap(),
            lower_right: Texture::from_encoded(
                ctx,
                resources::GRASS_LOWER_RIGHT,
            )
            .unwrap(),
            top_left: Texture::from_encoded(
                ctx,
                resources::GRASS_TOP_LEFT,
            )
            .unwrap(),
            top: Texture::from_encoded(
                ctx,
                resources::GRASS_TOP,
            )
            .unwrap(),
            top_right: Texture::from_encoded(
                ctx,
                resources::GRASS_TOP_RIGHT,
            )
            .unwrap(),
            right: Texture::from_encoded(
                ctx,
                resources::GRASS_RIGHT,
            )
            .unwrap(),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for x in (32..725).step_by(75) {
            self.lower.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(x as f32, 768.0)),
            );
        }

        for y in (32..768).step_by(32) {
            self.left.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(0.0, y as f32)),
            );
        }

        for y in (32..768).step_by(32) {
            self.right.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(768., y as f32)),
            );
        }

        for x in (32..770).step_by(70) {
            self.top.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(x as f32, 0.0)),
            );
        }

        self.lower_left.draw(
            ctx,
            DrawParams::new().position(Vec2::new(0.0, 768.0)),
        );
        self.lower_right.draw(
            ctx,
            DrawParams::new().position(Vec2::new(768.0, 768.0)),
        );
        self.top_left.draw(
            ctx,
            DrawParams::new().position(Vec2::new(0., 0.)),
        );
        self.top_right.draw(
            ctx,
            DrawParams::new().position(Vec2::new(768., 0.0)),
        );
    }
}

const ROCK_1_POINTS: [Vec2<f32>; 4] = [
    Vec2::new(50.0, 60.),
    Vec2::new(150.0, 260.),
    Vec2::new(350.0, 260.),
    Vec2::new(50.0, 660.),
];

const ROCK_2_POINTS: [Vec2<f32>; 3] = [
    Vec2::new(120.0, 502.),
    Vec2::new(320.0, 132.),
    Vec2::new(650.0, 660.),
];

const GRASS_1_POINTS: [Vec2<f32>; 6] = [
    Vec2::new(150.0, 160.),
    Vec2::new(220.0, 120.),
    Vec2::new(290.0, 520.),
    Vec2::new(720.0, 730.),
    Vec2::new(750.0, 260.),
    Vec2::new(675.0, 360.),
];

const GRASS_2_POINTS: [Vec2<f32>; 4] = [
    Vec2::new(320.0, 520.),
    Vec2::new(520.0, 530.),
    Vec2::new(150.0, 260.),
    Vec2::new(575.0, 660.),
];

impl Background {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            grass: Grass::load(ctx),
            grass_1: Texture::from_encoded(
                ctx,
                resources::GRASS_DETAIL_1,
            )
            .unwrap(),
            grass_2: Texture::from_encoded(
                ctx,
                resources::GRASS_DETAIL_2,
            )
            .unwrap(),
            rock_1: Texture::from_encoded(ctx, resources::ROCK1)
                .unwrap(),
            rock_2: Texture::from_encoded(ctx, resources::ROCK2)
                .unwrap(),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let scale = Vec2::new(1.5, 1.5);

        graphics::clear(
            ctx,
            Color::rgb(
                118.0 / 255.0,
                197.0 / 255.0,
                100.0 / 255.0,
            ),
        );

        self.grass.draw(ctx);

        for point in &GRASS_1_POINTS {
            self.grass_1.draw(
                ctx,
                DrawParams::new()
                    .position(*point)
                    .scale(scale)
                    .color(Color::BLUE),
            );
        }

        for point in &GRASS_2_POINTS {
            self.grass_2.draw(
                ctx,
                DrawParams::new()
                    .position(*point)
                    .scale(scale)
                    .color(Color::BLUE),
            );
        }

        for point in &ROCK_1_POINTS {
            self.rock_1.draw(
                ctx,
                DrawParams::new().position(*point).scale(scale),
            );
        }

        for point in &ROCK_2_POINTS {
            self.rock_2.draw(
                ctx,
                DrawParams::new().position(*point).scale(scale),
            );
        }
    }
}
