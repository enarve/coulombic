use macroquad::prelude::*;

struct Hero {
    x: f32,
    y: f32,
    size: f32,
    size_effect_value: f32,
    speed: f32
}

impl Hero {
    fn visible_size(&self) -> f32 {
        self.size * self.size_effect_value
    }
}

#[macroquad::main("Coulombic")]
async fn main() {
    
    let mut hero = Hero {x: screen_width() / 2.0, y: screen_height() / 2.0, size: 10.0, size_effect_value: 1.0, speed: 3.0};
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

        // println!("Hero {} {}", hero.x, hero.y);
        
        draw_hero(&hero);
        next_frame().await   
    }
}

fn draw_hero(hero: &Hero) {
    draw_circle(hero.x, hero.y, hero.visible_size(), WHITE);
}
