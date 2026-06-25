use macroquad::prelude::*;

mod models;
use models::{CoulombicModel};

use crate::models::Particle;

#[macroquad::main("Coulombic")]
async fn main() {

    let mut model = CoulombicModel::new(screen_width(), screen_height());
    let protagonist = &mut model.protagonist;
    let particles = &model.particles;
    
    loop {
        // MARK: Inputs

        // Effect
        if is_key_pressed(KeyCode::Space) {
            println!("Ba-dashsh!!!");
            protagonist.size_effect = 3.0;
        }
        if is_key_released(KeyCode::Space) {
            protagonist.size_effect = 1.0;
        }

        // Movement
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            protagonist.coords.y -= protagonist.speed;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            protagonist.coords.y += protagonist.speed;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            protagonist.coords.x -= protagonist.speed;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            protagonist.coords.x += protagonist.speed;
        }
        
        draw_protagonist(&protagonist);
        next_frame().await   
    }
}

fn draw_protagonist(particle: &Particle) {
    draw_circle(particle.coords.x, particle.coords.y, particle.visible_size(), WHITE);
}

fn draw_particle(particle: &Particle) {
    const ENEMY_SIZE: f32 = 5.0;
    draw_circle(particle.coords.x, particle.coords.y, ENEMY_SIZE, BLUE)
}
