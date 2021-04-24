use tetra::{graphics::Rectangle, math::Vec2};

use crate::{fireball::Fireball, humanoid::Humanoid};

pub fn player_collided_with_enemy(player_pos: Vec2<f32>, enemies: &[Humanoid]) -> bool {
    let player_rect = Rectangle::new(player_pos.x, player_pos.y, 16.0, 16.0);
    let enemy_rects = enemy_rects(enemies);
    for enemy_rect in &enemy_rects {
        if player_rect.intersects(&enemy_rect) {
            return true;
        }
    }
    false
}

pub fn enemy_rects(enemies: &[Humanoid]) -> Vec<Rectangle> {
    enemies
        .iter()
        .map(|e| e.get_position())
        .map(Vec2::into_tuple)
        .map(|(x, y)| Rectangle::new(x, y, 16.0, 16.0))
        .collect()
}