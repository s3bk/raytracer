#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(self) -> f32 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(mut self) -> Vector3 {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, mut vector: Vector3) -> Vector3 {
        vector.x *= self;
        vector.y *= self;
        vector.z *= self;
        vector
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(mut self, factor: f32) -> Self {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
        self
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
        self
    }
}

#[cfg(test)]
macro_rules! assert_near_eq {
    ($expected:expr, $actual: expr) => {
        let diff = ($actual - $expected).abs();
        assert!(diff < std::f32::EPSILON);
    };
}

#[test]
fn dot() {
    assert_near_eq!(
        34.0,
        Vector3::new(20.0, -3.0, 3.0).dot(Vector3::new(5.0, 20.0, -2.0))
    );
    assert_near_eq!(
        229.0,
        Vector3::new(-3.0, 2.0, -13.0).dot(Vector3::new(-9.0, -16.0, -18.0))
    );
    assert_near_eq!(
        430.0,
        Vector3::new(7.0, -9.0, -19.0).dot(Vector3::new(18.0, -19.0, -7.0))
    );
}
