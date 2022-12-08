use std::time::Duration;

use rand::{
    distributions::Uniform, prelude::Distribution,
    seq::SliceRandom, Rng,
};
use tetra::{graphics::Rectangle, math::Vec2, Context};

use crate::{
    animation::CannonballAnimation,
    debug_println,
    humanoid::{Humanoid, HumanoidType},
    oneoffanim::OneOffAnimationManager,
    projectile::{Projectile, ProjectileManager},
    textures::GruntTextures,
    timer::Timer,
    traits::Cleanable,
};

pub struct EnemyManager {
    /// All enemies currently spawned
    pub enemies: Vec<Humanoid>,
    /// Times the interval in which enemies can be spawned
    spawn_timer: Timer,
    /// Average enemy velocity
    avg_enemy_vel: f32,
    /// Spawns and cleans up projectiles coming from enemies
    projectile_mgr: ProjectileManager,
    /// All enemy textures already loaded into memory
    textures: GruntTextures,
}

impl Cleanable for EnemyManager {
    fn clean_up(&mut self) {
        let enemies_before = self.enemies.len();

        self.enemies.retain(|enemy| !enemy.is_dead());

        if self.enemies.len() < enemies_before {
            debug_println!(
                "[LOG] {} enemies dropped",
                enemies_before - self.enemies.len()
            );
        }
    }
}

impl EnemyManager {
    pub fn new(ctx: &mut Context) -> Self {
        let cannonball_animation =
            CannonballAnimation::build(ctx);

        Self {
            enemies: Vec::with_capacity(24),
            avg_enemy_vel: 1.0,
            spawn_timer: Timer::start_now_with_interval(
                Duration::from_secs_f64(1.5),
            ),
            projectile_mgr: ProjectileManager::new(
                cannonball_animation,
            ),
            textures: GruntTextures::load(ctx),
        }
    }

    fn generate_spawn_location<R: Rng>(
        rng: &mut R,
    ) -> (f32, f32) {
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

    pub fn calc_score(&self) -> u64 {
        let enemy_score = |kind| match kind {
            HumanoidType::BasicEnemy => 100,
            HumanoidType::StrongerEnemy => 250,
            HumanoidType::BadassEnemy => 500,
            HumanoidType::Boss => 1250,
            HumanoidType::Player => unreachable!(),
        };

        self.enemies
            .iter()
            .map(|e| e.kind())
            .map(enemy_score)
            .sum()
    }

    pub fn spawn_enemy<R: Rng>(
        &mut self,
        kind: HumanoidType,
        rng: &mut R,
    ) {
        self.spawn_timer.reset();

        let (lives, allowed_to_shoot, shooting_wait_time) =
            match kind {
                HumanoidType::Player => unreachable!(),
                HumanoidType::BasicEnemy => {
                    (1, false, Duration::from_secs_f32(0.0))
                }
                HumanoidType::StrongerEnemy => {
                    (2, true, Duration::from_secs_f32(1.0))
                }
                HumanoidType::BadassEnemy => {
                    (3, true, Duration::from_secs_f32(0.25))
                }
                HumanoidType::Boss => {
                    (10, true, Duration::from_secs_f32(0.10))
                }
            };

        let texture =
            self.textures.choose_enemy_from_kind(kind, rng);

        let enemy_vel = Vec2::new(
            rng.gen_range(0.3..0.7) + self.avg_enemy_vel,
            rng.gen_range(0.3..0.7) + self.avg_enemy_vel,
        );

        self.avg_enemy_vel +=
            (enemy_vel.x + enemy_vel.y) / 256.0;

        let (x, y) = Self::generate_spawn_location(rng);

        let enemy = Humanoid::new(
            lives,
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
        self.spawn_timer.is_ready()
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        player_pos: Vec2<f32>,
    ) {
        // Clean up dead enemies
        self.clean_up();

        self.projectile_mgr.advance_animation(ctx);

        for enemy in &mut self.enemies {

            let (
                is_fast_shooting,
                is_fast_running,
                is_triple_shooting,
            ) = enemy.power_ups.currently_active();
            let velocity = if is_fast_shooting {
                Vec2 { x: 7.5, y: 7.5 }
            } else {
                Vec2 { x: 4.5, y: 4.5 }
            };

            if enemy.can_fire() {
                let angle_to_player_deg =
                    enemy.angle_to_pos(player_pos).to_degrees();
                self.projectile_mgr.shoot(
                    is_triple_shooting,
                    angle_to_player_deg,
                    enemy.position,
                    velocity,
                );
                enemy.shooting_behavior.register_fire();
            }

            // Advance the animation of all enemies and update
            // their locations
            enemy.advance_animation(ctx);
            enemy.head_to(is_fast_running, player_pos);
        }
    }

    pub fn check_for_fireball_collisions(
        &mut self,
        enemy_rects: &[Rectangle],
        fireballs: &[Projectile],
        one_off_anim_mgr: &mut OneOffAnimationManager,
    ) {
        let fireball_rects = fireballs
            .iter()
            .map(Projectile::position)
            .map(Vec2::into_tuple)
            .map(|(x, y)| {
                Rectangle::new(x + 5.0, y + 5.0, 32.0, 32.0)
            });

        for fireball in fireball_rects {
            for (enemy, enemy_rect) in
                self.enemies.iter_mut().zip(enemy_rects)
            {
                if enemy_rect.intersects(&fireball) {
                    let (x, y) =
                        (fireball.x + 5.0, fireball.y + 5.0);
                    one_off_anim_mgr
                        .add_explosion(Vec2 { x, y });

                    enemy.take_hit();
                }
            }
        }
    }

    pub fn check_for_cannonball_collisions(
        &self,
        player: &mut Humanoid,
        one_off_anim_mgr: &mut OneOffAnimationManager,
    ) {
        let player_rect = player.rectangle();
        for cannon in self.projectile_mgr.projectiles() {
            let cannon_pos = cannon.position();
            let cannon_rect = Rectangle::new(
                cannon_pos.x,
                cannon_pos.y,
                16.0,
                16.0,
            );
            if cannon_rect.intersects(&player_rect) {
                player.take_hit();
                one_off_anim_mgr.add_smoke(cannon_pos);
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for enemy in self.enemies.iter_mut() {
            enemy.draw(ctx);
        }
        self.projectile_mgr.draw(ctx);
    }
}
