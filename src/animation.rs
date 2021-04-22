use std::time::Duration;

use tetra::graphics::animation::Animation;
use tetra::graphics::Rectangle;
use tetra::graphics::Texture;

/// Animation for the player and grunts
pub struct HumanoidAnimation {
    // Front-side walking animation
    pub frontside: Animation,
    // Back-side walking animation
    pub backside: Animation,
    // "Left-side" walking animation
    pub leftside: Animation,
}

impl HumanoidAnimation {
    pub fn new(texture: Texture) -> Self {
        let frontside = Animation::new(
            texture.clone(),
            vec![
                Rectangle::new(0., 0., 16., 16.),
                Rectangle::new(48., 0., 16., 16.),
            ],
            Duration::from_secs_f64(0.5),
        );

        let backside = Animation::new(
            texture.clone(),
            vec![
                Rectangle::new(16., 0., 16., 16.),
                Rectangle::new(64., 0., 16., 16.),
            ],
            Duration::from_secs_f64(0.5),
        );

        let leftside = Animation::new(
            texture,
            vec![
                Rectangle::new(32., 0., 16., 16.),
                Rectangle::new(80., 0., 16., 16.),
            ],
            Duration::from_secs_f64(0.5),
        );

        Self {
            frontside,
            leftside,
            backside,
        }
    }
}

pub struct FireballAnimation {
    animation: Animation,
}

impl FireballAnimation {
    pub fn new (texture: Texture) -> Self {
        let animation = Animation::new(
            texture,
            vec![
                Rectangle::new(0., 0., 16., 16.),
                Rectangle::new(16., 0., 16., 16.),
            ],
            Duration::from_secs_f64(0.25)
        );

        Self {
            animation
        }
    }
}