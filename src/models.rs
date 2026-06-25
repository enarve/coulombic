pub struct Coordinates {
    pub x: f32,
    pub y: f32
}

pub struct Particle {
    pub coords: Coordinates,
    pub charge: f32,
    pub size_effect: f32,
    pub speed: f32
}

impl Particle {
    pub fn visible_size(&self) -> f32 {
        self.charge.abs() * self.size_effect
    }
}

pub struct CoulombicModel {
    pub protagonist: Particle,
    pub particles: Vec<Particle>
}

impl CoulombicModel {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let protagonist = Particle {
            coords: Coordinates { x: screen_width / 2.0, y: screen_height / 2.0 },
            charge: 1.0,
            size_effect: 1.0,
            speed: 2.0
        };
        return CoulombicModel {
            protagonist: protagonist,
            particles: vec![]
        }
    }
}