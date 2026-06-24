use macroquad::prelude::*;

mod models;
use models::{Hero, Enemy};

#[macroquad::main("Coulombic")]
async fn main() {
    
    let mut hero = Hero {x: screen_width() / 2.0, y: screen_height() / 2.0, size: 10.0, size_effect_value: 1.0, speed: 3.0};
    let enemy = Enemy {x: 300.0, y: 300.0, charge: 1.0 };
    
    loop {
        // MARK: Inputs

        // Effect
        if is_key_pressed(KeyCode::Space) {
            println!("Ba-dashsh!!!");
            hero.size_effect_value = 3.0;
        }
        if is_key_released(KeyCode::Space) {
            hero.size_effect_value = 1.0;
        }

        // Movement
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            hero.y -= hero.speed;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            hero.y += hero.speed;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            hero.x -= hero.speed;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            hero.x += hero.speed;
        }
        
        draw_hero(&hero);
        draw_enemy(&enemy);
        next_frame().await   
    }
}

fn draw_hero(hero: &Hero) {
    draw_circle(hero.x, hero.y, hero.visible_size(), WHITE);
}

fn draw_enemy(enemy: &Enemy) {
    const ENEMY_SIZE: f32 = 5.0;
    draw_circle(enemy.x, enemy.y, ENEMY_SIZE, BLUE)
}
