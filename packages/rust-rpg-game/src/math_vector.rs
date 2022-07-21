use std::ops::Sub;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let angle = self.angle_from(other);
        let z = angle.sin() * self.magnitude() * other.magnitude();

        Vec3 {
            z,
            ..Default::default()
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle_from(&self, other: &Vec3) -> f64 {
        (self.dot(other) / self.magnitude() / other.magnitude()).acos()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl From<(f64, f64)> for Vec3 {
    fn from((x, y): (f64, f64)) -> Self {
        Vec3 {
            x,
            y,
            ..Default::default()
        }
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vec3 { x, y, z }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::FRAC_PI_2;

    use super::*;

    #[test]
    fn test_mag() {
        let v: Vec3 = (3.0, 4.0).into();

        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_dot_product() {
        let v1: Vec3 = (1.0, 2.0, 3.0).into();
        let v2: Vec3 = (1.0, 5.0, 7.0).into();

        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_angle() {
        let v1: Vec3 = (0.0, 2.0).into();
        let v2: Vec3 = (1.0, 0.0).into();

        assert_eq!(v1.angle_from(&v2), FRAC_PI_2);
    }

    #[test]
    fn test_cross_product() {
        let v1: Vec3 = (-1.0, -2.0).into();
        let v2: Vec3 = (4.0, 0.0).into();

        assert_eq!(v1.cross(&v2), (0.0, 0.0, 8.0).into());
    }
}
