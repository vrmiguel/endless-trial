mod animation;
mod background;
mod bounds;
mod direction;
mod enemy;
mod gamestate;
mod healthbar;
mod humanoid;
mod macros;
mod oneoffanim;
mod panel;
mod powerup;
mod projectile;
mod resources;

use bounds::Bounds;
use direction::Direction;
use gamestate::GameState;
use tetra::ContextBuilder;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const BOUNDS: Bounds = Bounds::new(800.0, 800.0);

const VERSION: &str = "0.1.0";

fn main() -> tetra::Result {
    println!("Endless Trial v{}", VERSION);
    ContextBuilder::new("Endless Trial", WIDTH, HEIGHT)
        .quit_on_escape(true)
        .debug_info(true)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
