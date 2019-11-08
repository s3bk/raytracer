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

    pub fn add(&mut self, target: Color, amount: f32) {
        self.r += target.r * amount;
        self.g += target.g * amount;
        self.b += target.b * amount;
    }

    pub fn add_ambient(&mut self, ambient: Color) {
        self.add(ambient, 1.0);
    }

    pub fn add_directional(&mut self, color: Color, strength: f32) {
        debug_assert!(strength >= 0.0 && strength <= 1.0);
        self.add(color, strength);
    }
}

impl std::ops::Add for Color {
    type Output = Color;
    fn add(mut self, other: Self) -> Self {
        self.add(other);
        self
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
impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}
