use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.z,
        )
    }

    pub fn unit(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z);
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        *self = Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z);
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        *self = Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z);
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, v: Vec3) {
        *self = Vec3::new(self.x / v.x, self.y / v.y, self.z / v.z);
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Vec3::new(self.x * t, self.y * t, self.z * t);
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = Vec3::new(self.x / t, self.y / t, self.z / t);
    }
}
