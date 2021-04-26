use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use rand::{distributions::Uniform, prelude::Distribution, rngs::StdRng};
use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;

use crate::{
    humanoid::{Humanoid, HumanoidType},
    animation::CannonballAnimation,
    oneoffanim::OneOffAnimationManager,
    projectile::{Fireball, ProjectileManager},
};
use crate::{
    resources::{BASIC_GRUNTS, STRONGER_GRUNTS},
    BOUNDS,
};

use crate::debug_println;

pub struct EnemyManager {
    enemies: Vec<Humanoid>,
    last_spawn_time: Instant,
    avg_enemy_vel: f32,
    projectile_mgr: ProjectileManager,
}

impl EnemyManager {
    pub fn new(ctx: &mut Context) -> Self {

        let cannonball_animation = CannonballAnimation::make_animation(ctx);

        Self {
            enemies: vec![],
            last_spawn_time: Instant::now(),
            avg_enemy_vel: 1.0,
            projectile_mgr: ProjectileManager::new(cannonball_animation),
        }
    }

    fn generate_spawn_location(rng: &mut StdRng) -> (f32, f32) {
        // These unwraps here are all safe
        let boundary = *[0., 800.].choose(rng).unwrap();

        let range = Uniform::from(0. ..=800.);
        let pos = range.sample(rng);

        if rng.gen_bool(0.5) {
            (pos, boundary)
        } else {
            (boundary, pos)
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
            HumanoidType::StrongerEnemy => STRONGER_GRUNTS
                .choose(&mut rng)
                .expect("STRONGER_GRUNTS should not be empty"),
            HumanoidType::BadassEnemy => todo!(),
            HumanoidType::Boss => todo!(),
        };

        let (allowed_to_shoot, shooting_wait_time) = match kind {
            HumanoidType::Player => unreachable!(),
            HumanoidType::BasicEnemy => (false, Duration::from_secs_f32(0.0)),
            HumanoidType::StrongerEnemy => (true, Duration::from_secs_f32(1.0)),
            HumanoidType::BadassEnemy => (true, Duration::from_secs_f32(0.5)),
            HumanoidType::Boss => (true, Duration::from_secs_f32(0.20)),
        };

        let texture = Texture::from_file_data(ctx, sprite).expect("failed to load built-in sprite");

        // rng.gen_range(0.0..10.0)
        let enemy_vel = Vec2::new(
            rng.gen_range(0.3..0.7) + self.avg_enemy_vel,
            rng.gen_range(0.3..0.7) + self.avg_enemy_vel,
        );

        self.avg_enemy_vel += (enemy_vel.x + enemy_vel.y) / 64.0;

        let (x, y) = Self::generate_spawn_location(&mut rng);

        let enemy = Humanoid::new(
            texture,
            Vec2::new(x, y),
            enemy_vel,
            allowed_to_shoot,
            shooting_wait_time,
            kind,
        );
        self.enemies.push(enemy);
    }

    pub fn can_spawn(&self) -> bool {
        let time_since_last_spawn = self.last_spawn_time.elapsed();

        time_since_last_spawn > Duration::from_secs_f64(1.5)
    }

    pub fn clean_up_oob(&mut self) {
        let enemies_before = self.enemies.len();
        self.enemies.retain(|enemy| BOUNDS.contains(enemy.position));
        if self.enemies.len() < enemies_before {
            debug_println!(
                "[LOG] {} enemies dropped",
                enemies_before - self.enemies.len()
            );
        }
    }

    pub fn update(&mut self, ctx: &mut Context, player_pos: Vec2<f32>) {
        // Remove enemies that are out of bounds (i.e., dead enemies)
        self.clean_up_oob();

        for enemy in &mut self.enemies {
            if enemy.allowed_to_shoot && enemy.can_fire() {
                let angle_to_player = enemy.angle_to_pos(player_pos);
                self.projectile_mgr.add_projectile(angle_to_player, enemy.position);
                enemy.register_fire();
            }

            // Advance the animation of all enemies and update their locations
            enemy.advance_animation(ctx);
            enemy.head_to(player_pos);
        }
    }

    // Currently O(n²) :C
    pub fn check_for_fireball_collisions(
        &mut self,
        enemy_rects: &[Rectangle],
        fireballs: &[Fireball],
        one_off_anim_mgr: &mut OneOffAnimationManager,
    ) {
        let fireball_rects: Vec<_> = fireballs
            .iter()
            .map(|x| x.get_position())
            .map(Vec2::into_tuple)
            .map(|(x, y)| Rectangle::new(x + 5.0, y + 5.0, 32.0, 32.0))
            .collect();

        // Enemies that get hit with a fireball will be internally teleported somewhere far away so that our out-of-bounds system removes them
        let thrown_away_pos = Vec2::new(5000.0, 5000.0);

        for (enemy, enemy_rect) in self.enemies.iter_mut().zip(enemy_rects) {
            for fireball in &fireball_rects {
                if enemy_rect.intersects(fireball) {
                    let (x, y) = (fireball.x + 5.0, fireball.y + 5.0);
                    one_off_anim_mgr.add_explosion(Vec2 { x, y });
                    enemy.position = thrown_away_pos;
                }
            }
        }
    }

    pub fn enemies_ref(&self) -> &[Humanoid] {
        &*self.enemies
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for enemy in self.enemies.iter_mut() {
            enemy.draw(ctx);
        }
    }
}
