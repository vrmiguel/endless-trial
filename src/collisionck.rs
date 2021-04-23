use tetra::{graphics::Rectangle, math::Vec2};

use crate::{fireball::Fireball, humanoid::Humanoid};

pub fn player_collided_with_enemy(player_pos: Vec2<f32>, enemies: &[Humanoid]) -> bool {
    let player_rect = Rectangle::new(player_pos.x, player_pos.y, 16.0, 16.0);
    for enemy in enemies {
        let enemy_pos = enemy.get_position();
        let enemy_rect = Rectangle::new(enemy_pos.x, enemy_pos.y, 16.0, 16.0);
        if player_rect.intersects(&enemy_rect) {
            return true;
        }
    }
    false
}

pub fn enemy_collided_with_fireball(enemies: &[Humanoid], fireballs: Fireball) {
    
}