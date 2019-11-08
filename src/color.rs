#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn change_towards(&mut self, target: Color, amount: f32) {
        debug_assert!(amount >= 0.0 && amount <= 1.0);
        let diff_r = target.r - self.r;
        let diff_g = target.g - self.g;
        let diff_b = target.b - self.b;

        self.r += diff_r * amount;
        self.g += diff_g * amount;
        self.b += diff_b * amount;
    }

    pub fn add_ambient(&mut self, ambient: Color) {
        self.change_towards(ambient, 0.1);
    }

    pub fn add_directional(&mut self, color: Color, strength: f32) {
        debug_assert!(strength >= 0.0 && strength <= 1.0);
        self.change_towards(color, strength);
    }
}

impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r.max(other.r),
            g: self.g.max(other.g),
            b: self.b.max(other.b),
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Color;
    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}
