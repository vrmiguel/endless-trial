mod animation;
mod background;
mod direction;
mod humanoid;
mod fireball;
mod sprites;
mod gamestate;
mod macros;

use tetra::ContextBuilder;

use direction::Direction;
use gamestate::GameState;


const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const DEG_TO_RAD: f32 = 3.14159265358979323846/180.0;

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
