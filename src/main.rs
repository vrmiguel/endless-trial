mod animation;
mod background;
mod direction;
mod enemy;
mod fireball;
mod gamestate;
mod healthbar;
mod humanoid;
mod macros;
mod panel;
mod powerup;
mod resources;
mod bounds;

use bounds::Bounds;
use tetra::ContextBuilder;

use direction::Direction;
use gamestate::GameState;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const DEG_TO_RAD: f32 = 3.14159265358979323846 / 180.0;
const RAD_TO_DEG: f32 = 180.0 / 3.14159265358979323846;
const BOUNDS: Bounds = Bounds::new(800.0, 800.0);

const VERSION: &str = "0.1.0";

fn main() -> tetra::Result {
    println!("joguinho v{}", VERSION);
    ContextBuilder::new("my lil game", WIDTH, HEIGHT)
        .quit_on_escape(true)
        .debug_info(true)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
