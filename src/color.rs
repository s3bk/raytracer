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

    fn change_torwards(&mut self, target: Color, amount: f32) {
        let diff_r = target.r - self.r;
        self.r += (diff_r * amount).min(0.0).max(1.0);
        let diff_g = target.g - self.g;
        self.g += (diff_g * amount).min(0.0).max(1.0);
        let diff_b = target.b - self.b;
        self.b += (diff_b * amount).min(0.0).max(1.0);
    }

    pub fn add_ambient(&mut self, ambient: Color) {
        self.change_torwards(ambient, 0.01);
    }

    pub fn add_directional(&mut self, color: Color, strength: f32) {
        debug_assert!(strength >= 0.0 && strength <= 1.0);
        self.change_torwards(color, strength);
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
