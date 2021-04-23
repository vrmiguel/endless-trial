use std::time::{Duration, Instant};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use tetra::graphics::Texture;
use tetra::math::Vec2;
use tetra::Context;

use crate::humanoid::{Humanoid, HumanoidType};
use crate::sprites::BASIC_GRUNTS;

pub struct EnemyManager {
    enemies: Vec<Humanoid>,
    last_spawn_time: Instant,
    avg_enemy_vel: f32
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: vec![],
            last_spawn_time: Instant::now(),
            avg_enemy_vel: 1.0
        }
    }

    pub fn spawn_enemy(&mut self, ctx: &mut Context, kind: HumanoidType) {
        let mut rng = StdRng::from_entropy();

        self.last_spawn_time = Instant::now();
        let sprite = match kind {
            HumanoidType::Player => panic!("An enemy cannot have the player's sprite"),
            HumanoidType::BasicEnemy => BASIC_GRUNTS
                .choose(&mut rng)
                .expect("BASIC_GRUNTS should not be empty"),
            HumanoidType::StrongerEnemy => todo!(),
            HumanoidType::BadassEnemy => todo!(),
            HumanoidType::Boss => todo!(),
        };

        let texture = Texture::from_file_data(ctx, sprite).expect("failed to load built-in sprite");

        // rng.gen_range(0.0..10.0)
        let enemy_vel = Vec2::new(
            rng.gen_range(0.5 .. 1.0) + self.avg_enemy_vel,
            rng.gen_range(0.5 .. 1.0) + self.avg_enemy_vel
        );

        self.avg_enemy_vel += (enemy_vel.x + enemy_vel.y)/16.0;

        let enemy = Humanoid::new(texture, Vec2::new(0., 0.), enemy_vel, kind);
        self.enemies.push(enemy);
    }

    pub fn can_spawn(&self) -> bool {
        let time_since_last_spawn = self.last_spawn_time.elapsed();

        time_since_last_spawn > Duration::from_secs_f64(1.5) 
    }

    pub fn update(&mut self, heading_to: Vec2<f32>) {
        for enemy in &mut self.enemies {
            enemy.head_to(heading_to);
        }
    }

    pub fn draw(& self, ctx: &mut Context) {
        for enemy in &self.enemies {
            enemy.draw(ctx);
        }
    }
}
