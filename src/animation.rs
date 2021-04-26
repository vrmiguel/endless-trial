use std::time::Duration;

use tetra::graphics::Rectangle;
use tetra::graphics::Texture;
use tetra::{graphics::animation::Animation, Context};

use crate::resources::{CANNONBALL, FIREBALL};

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

pub struct FireballAnimation;

impl FireballAnimation {
    pub fn make_animation(ctx: &mut Context) -> Animation {
        let fireball_texture =
            Texture::from_file_data(ctx, FIREBALL).expect("couldn't read the fireball sprite");
        Animation::new(
            fireball_texture,
            Rectangle::row(0., 0., 32., 32.).take(5).collect(),
            Duration::from_secs_f64(0.10),
        )
    }
}

pub struct CannonballAnimation;

impl CannonballAnimation {
    pub fn make_animation(ctx: &mut Context) -> Animation {
        let cannonball_texture =
            Texture::from_file_data(ctx, CANNONBALL).expect("couldn't read the cannonball sprite");

        Animation::new(
            cannonball_texture,
            Rectangle::row(0., 0., 32., 32.).take(5).collect(),
            Duration::from_secs_f64(0.10),
        )
    }
}
