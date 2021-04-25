use tetra::{graphics::Rectangle, math::Vec2};

pub struct Bounds {
    boundaries: Rectangle,
}

impl Bounds {
    pub const fn new (width: f32, height: f32) -> Self {
        Self {
            boundaries: Rectangle::new(0.0, 0.0, width, height)
        }
    }

    pub fn contains(&self, pos: Vec2<f32>) -> bool {
        pos.x <= self.boundaries.width && pos.y <= self.boundaries.height && pos.x >= 0. && pos.y >= 0.
    }
}