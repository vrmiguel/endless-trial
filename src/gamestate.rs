use std::time::{Duration, Instant};

use rand::{
    prelude::{SliceRandom, SmallRng},
    SeedableRng,
};
use tetra::{
    graphics,
    graphics::scaling::{ScalingMode, ScreenScaler},
    input::{self, Key},
    time, window, Context, Event, State,
};

use crate::{
    background::Background, enemy::EnemyManager,
    healthbar::HealthBar, humanoid::HumanoidType,
    oneoffanim::OneOffAnimationManager, panel::GameOverPanel,
    player::PlayerManager, powerup::PowerUpManager,
    timer::Timer, HEIGHT, WIDTH,
};

/// Enemy types and their spawn rate percentages for each wave
const WAVES: &[[(HumanoidType, f32); 4]] = &[
    [
        (HumanoidType::BasicEnemy, 0.85),
        (HumanoidType::StrongerEnemy, 0.10),
        (HumanoidType::BadassEnemy, 0.05),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.75),
        (HumanoidType::StrongerEnemy, 0.20),
        (HumanoidType::BadassEnemy, 0.05),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.75),
        (HumanoidType::StrongerEnemy, 0.20),
        (HumanoidType::BadassEnemy, 0.05),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.4),
        (HumanoidType::StrongerEnemy, 0.5),
        (HumanoidType::BadassEnemy, 0.1),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.1),
        (HumanoidType::StrongerEnemy, 0.6),
        (HumanoidType::BadassEnemy, 0.3),
        (HumanoidType::Boss, 0.0),
    ],
    [
        (HumanoidType::BasicEnemy, 0.0),
        (HumanoidType::StrongerEnemy, 0.55),
        (HumanoidType::BadassEnemy, 0.35),
        (HumanoidType::Boss, 0.1),
    ],
    [
        (HumanoidType::BasicEnemy, 0.0),
        (HumanoidType::StrongerEnemy, 0.55),
        (HumanoidType::BadassEnemy, 0.25),
        (HumanoidType::Boss, 0.2),
    ],
];

pub struct GameState {
    /// The active screen scaler
    scaler: ScreenScaler,
    /// The textures of the game's background
    background: Background,
    health_bar: HealthBar,
    player_manager: PlayerManager,
    power_up_mgr: PowerUpManager,
    enemy_mgr: EnemyManager,
    one_off_anim_mgr: OneOffAnimationManager,
    game_over_panel: GameOverPanel,
    rng: SmallRng,
    game_score: u64,
    current_wave: u8,
    /// How long every wave lasts
    wave_timer: Timer,
    window_title_update_timer: Timer,
    #[cfg(debug_assertions)]
    diagnostics: Diagnostics,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let now = Instant::now();

        let game_state = GameState {
            player_manager: PlayerManager::new(ctx),
            background: Background::new(ctx),
            health_bar: HealthBar::new(ctx),
            power_up_mgr: PowerUpManager::new(ctx),
            scaler: ScreenScaler::with_window_size(
                ctx,
                WIDTH,
                HEIGHT,
                ScalingMode::ShowAll,
            )?,
            game_over_panel: GameOverPanel::new(ctx),
            enemy_mgr: EnemyManager::new(ctx),
            one_off_anim_mgr: OneOffAnimationManager::new(ctx),
            rng: SmallRng::from_entropy(),
            game_score: 0,
            current_wave: 0,
            wave_timer: Timer::start_now_with_interval(
                Duration::from_secs(30),
            ),
            #[cfg(debug_assertions)]
            diagnostics: Diagnostics::new(),
            window_title_update_timer:
                Timer::start_now_with_interval(
                    Duration::from_secs(1),
                ),
        };

        // How long we took to instantiate all textures into GPU
        // memory and build the managers
        println!(
            "Built initial GameState in {}ms",
            now.elapsed().as_millis()
        );

        Ok(game_state)
    }

    fn check_for_wave_change(&mut self) {
        if self.wave_timer.is_ready()
            && self.current_wave < (WAVES.len() as u8 - 1)
        {
            self.current_wave += 1;
            self.wave_timer.reset();
            println!(
                "Commencing wave {}",
                self.current_wave + 1
            );
        }
    }

    fn check_for_scale_change(&mut self, ctx: &mut Context) {
        if input::is_key_pressed(ctx, Key::F1) {
            let next = match self.scaler.mode() {
                ScalingMode::Fixed => ScalingMode::Stretch,
                ScalingMode::Stretch => ScalingMode::ShowAll,
                ScalingMode::ShowAll => {
                    ScalingMode::ShowAllPixelPerfect
                }
                ScalingMode::ShowAllPixelPerfect => {
                    ScalingMode::Crop
                }
                ScalingMode::Crop => {
                    ScalingMode::CropPixelPerfect
                }
                ScalingMode::CropPixelPerfect => {
                    ScalingMode::Fixed
                }
                _ => ScalingMode::Fixed,
            };

            println!("[LOG] Scaling mode changed to {next:?}");

            self.scaler.set_mode(next);
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.player_manager.is_player_dead()
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        #[cfg(debug_assertions)]
        self.diagnostics.start_polling();

        graphics::set_canvas(ctx, self.scaler.canvas());

        self.background.draw(ctx);

        self.player_manager.draw(ctx);
        self.enemy_mgr.draw(ctx);
        self.power_up_mgr.draw(
            ctx,
            &self.player_manager.player_mut().power_ups,
        );
        self.health_bar.draw(ctx, self.player_manager.hearts());
        self.one_off_anim_mgr.draw(ctx);

        if self.is_game_over() {
            self.game_over_panel.draw(ctx);
        }

        graphics::reset_canvas(ctx);
        self.scaler.draw(ctx);

        // Update the window title only once per second
        if self.window_title_update_timer.is_ready() {
            self.window_title_update_timer.reset();

            window::set_title(
                ctx,
                &format!(
                    "Endless Trial - {:.0} FPS - Wave: {} - Score: {}",
                    time::get_fps(ctx),
                    self.current_wave + 1,
                    self.game_score
                ),
            );
        }

        #[cfg(debug_assertions)]
        self.diagnostics.finish_polling(PollKind::Drawing);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        #[cfg(debug_assertions)]
        self.diagnostics.start_polling();

        // Checks if the player changed the screen scaling method
        self.check_for_scale_change(ctx);

        // Freeze the game logic if the game is over
        if self.is_game_over() {
            return Ok(());
        }

        // Checks if the current wave is over
        self.check_for_wave_change();

        // Check if the player collided with an enemy.
        //
        // We return `enemy_rects` here (Vec of Retangles for
        // each enemy) in order to reuse it in
        // `check_for_fireball_collisions`
        let (collided_with_an_enemy, enemy_rects) = self
            .player_manager
            .player_mut()
            .collided_with_bodies(&self.enemy_mgr.enemies);

        if collided_with_an_enemy {
            self.player_manager.register_hit();
        }

        // Check if an enemy was hit with a fireball from the
        // player
        self.enemy_mgr.check_for_fireball_collisions(
            &enemy_rects,
            self.player_manager.fireballs(),
            &mut self.one_off_anim_mgr,
        );

        // Check if the player was hit with a cannonball from an
        // enemy
        self.enemy_mgr.check_for_cannonball_collisions(
            self.player_manager.player_mut(),
            &mut self.one_off_anim_mgr,
        );

        // Check if any enemy got a power-up
        for enemy in self.enemy_mgr.enemies.iter_mut() {
            self.power_up_mgr.check_for_collision(enemy);
        }

        self.power_up_mgr.advance(
            &mut self.rng,
            self.player_manager.player_mut(),
        );

        self.player_manager.update(ctx);

        if self.enemy_mgr.can_spawn() {
            let kind = WAVES[self.current_wave as usize]
                .choose_weighted(&mut self.rng, |x| x.1)
                .expect("WAVES should not be empty")
                .0;
            self.enemy_mgr.spawn_enemy(kind, &mut self.rng);
        }

        // Calculate the enemy score now that new enemies have
        // been spawned
        let enemy_score = self.enemy_mgr.calc_score();

        self.one_off_anim_mgr.update();

        self.enemy_mgr
            .update(ctx, self.player_manager.player_position());

        self.power_up_mgr.update();

        // If the game score has decreased then enemies have been
        // killed, which adds to the game score
        self.game_score +=
            enemy_score - self.enemy_mgr.calc_score();

        #[cfg(debug_assertions)]
        self.diagnostics.finish_polling(PollKind::Update);

        Ok(())
    }

    fn event(
        &mut self,
        _ctx: &mut Context,
        event: Event,
    ) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }
}

#[cfg(debug_assertions)]
struct Diagnostics {
    /// How often we print the game diagnostics
    /// to stdout
    flush_timer: Timer,
    /// How much it took in average to draw a new frame
    /// during the last polling interval
    avg_draw_time: Duration,
    /// How much it took in average to update the game state
    /// during the last polling interval
    avg_update_time: Duration,
    current_polling_started: Instant,
    times_polled: u32,
}

#[cfg(debug_assertions)]
impl Diagnostics {
    pub fn new() -> Self {
        Self {
            flush_timer: Timer::start_now_with_interval(
                Duration::from_secs(15),
            ),
            avg_draw_time: Duration::new(0, 0),
            avg_update_time: Duration::new(0, 0),
            current_polling_started: Instant::now(),
            times_polled: 0,
        }
    }

    pub fn start_polling(&mut self) {
        self.current_polling_started = Instant::now()
    }

    pub fn finish_polling(&mut self, kind: PollKind) {
        self.times_polled += 1;

        let elapsed = self.current_polling_started.elapsed();

        if self.times_polled == 1 {
            match kind {
                PollKind::Drawing => {
                    self.avg_draw_time = elapsed
                }
                PollKind::Update => {
                    self.avg_update_time = elapsed
                }
            }
            return;
        }

        match kind {
            PollKind::Drawing => {
                self.avg_draw_time += elapsed;
            }
            PollKind::Update => {
                self.avg_update_time += elapsed;
            }
        }

        if self.flush_timer.is_ready() {
            self.flush_timer.reset();
            let avg_draw_time =
                self.avg_draw_time / self.times_polled;
            let avg_update_time =
                self.avg_update_time / self.times_polled;

            println!("Last {} frames: avg. draw {}ns, avg. update {}ns", self.times_polled, avg_draw_time.as_nanos(), avg_update_time.as_nanos());
        }
    }
}

#[cfg(debug_assertions)]
#[derive(Clone, Copy)]
enum PollKind {
    Drawing,
    Update,
}
