// Hero
pub struct Hero {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub size_effect_value: f32,
    pub speed: f32
}

impl Hero {
    pub fn visible_size(&self) -> f32 {
        self.size * self.size_effect_value
    }
}

// Enemy particle
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub charge: f32
}