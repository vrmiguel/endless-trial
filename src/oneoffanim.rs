use std::{
    time::{Duration, Instant},
    usize,
};

use tetra::{
    graphics::{
        animation::Animation, DrawParams, Rectangle, Texture,
    },
    math::Vec2,
    Context,
};

use crate::{
    resources::{EXPLOSION, SMOKE},
    traits::Cleanable,
};

struct OneOffAnimation {
    current_frame: u8,
    position: Vec2<f32>,
}

impl OneOffAnimation {
    pub fn new(position: Vec2<f32>) -> Self {
        Self {
            position,
            current_frame: 0,
        }
    }
}

pub struct OneOffAnimationManager {
    last_updated_time: Instant,
    last_smoke_added_time: Instant,
    last_explosion_added_time: Instant,
    explosion_anim: Animation,
    explosion_anim_frames: u8,
    smoke_anim: Animation,
    smoke_anim_frames: u8,
    explosions: Vec<OneOffAnimation>,
    smokes: Vec<OneOffAnimation>,
}

impl Cleanable for OneOffAnimationManager {
    /// Remove animations that have finished
    fn clean_up(&mut self) {
        let explosion_final_frame =
            self.explosion_anim_frames - 1;
        let smoke_final_frame = self.smoke_anim_frames - 1;

        self.explosions.retain(|x| {
            x.current_frame != explosion_final_frame
        });

        self.smokes
            .retain(|x| x.current_frame != smoke_final_frame);
    }
}

impl OneOffAnimationManager {
    pub fn new(ctx: &mut Context) -> Self {
        let explosion_sprite = Texture::from_encoded(
            ctx, EXPLOSION,
        )
        .expect("Failed to load built-in explosion sprite");

        let smoke_sprite = Texture::from_encoded(ctx, SMOKE)
            .expect("Failed to load built-in smoke sprite");

        let explosion_anim = Animation::new(
            explosion_sprite,
            Rectangle::row(0.0, 0.0, 64.0, 64.0)
                .take(10)
                .collect(),
            Duration::from_secs_f32(0.05),
        );

        let explosion_anim_frames =
            explosion_anim.frames().len() as u8;

        let smoke_anim = Animation::new(
            smoke_sprite,
            Rectangle::row(0.0, 0.0, 64.0, 64.0)
                .take(6)
                .collect(),
            Duration::from_secs_f32(0.05),
        );

        let smoke_anim_frames = smoke_anim.frames().len() as u8;

        Self {
            last_updated_time: Instant::now(),
            last_explosion_added_time: Instant::now(),
            last_smoke_added_time: Instant::now(),
            explosion_anim,
            explosion_anim_frames,
            smoke_anim,
            smoke_anim_frames,
            explosions: vec![],
            smokes: vec![],
        }
    }

    pub fn add_explosion(&mut self, position: Vec2<f32>) {
        if !self.can_add_explosion() {
            return;
        }
        self.last_explosion_added_time = Instant::now();
        let explosion_anim = OneOffAnimation::new(position);
        self.explosions.push(explosion_anim);
    }

    fn can_add_smoke(&self) -> bool {
        let elapsed = self.last_smoke_added_time.elapsed();

        elapsed > Duration::from_secs_f32(0.2)
    }

    fn can_add_explosion(&self) -> bool {
        let elapsed = self.last_explosion_added_time.elapsed();

        elapsed > Duration::from_secs_f32(0.2)
    }

    pub fn add_smoke(&mut self, position: Vec2<f32>) {
        if !self.can_add_smoke() {
            // Avoid spawning many smoke animations in the same
            // place (e.g. if the fireball hits the
            // enemy but he doesn't die)
            return;
        }
        self.last_smoke_added_time = Instant::now();
        let smoke_anim = OneOffAnimation::new(position);
        self.smokes.push(smoke_anim);
    }

    pub fn update(&mut self) {
        self.clean_up();

        if !self.can_update_frames() {
            return;
        }

        self.last_updated_time = Instant::now();

        for explosion in &mut self.explosions {
            explosion.current_frame += 1;
        }

        for smoke in &mut self.smokes {
            smoke.current_frame += 1;
        }
    }

    fn can_update_frames(&self) -> bool {
        let elapsed = self.last_updated_time.elapsed();

        elapsed > Duration::from_secs_f32(0.10)
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for explosion in &self.explosions {
            self.explosion_anim.set_current_frame_index(
                explosion.current_frame as usize,
            );
            self.explosion_anim.draw(
                ctx,
                DrawParams::new()
                    .position(explosion.position)
                    .origin(Vec2::new(16.0, 16.0)),
            );
        }

        for smoke in &self.smokes {
            self.smoke_anim.set_current_frame_index(
                smoke.current_frame as usize,
            );
            self.smoke_anim.draw(
                ctx,
                DrawParams::new()
                    .position(smoke.position)
                    .origin(Vec2::new(16.0, 16.0)),
            );
        }
    }
}
