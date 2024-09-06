use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub, SubAssign};
use std::str::from_utf8;

/// A vector in 3 coordinates
///
/// Mathematically, it can represent equally a 3d vector or a 3d point
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }
    
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }
    
    pub fn unit_z() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }
    
    pub fn newf(xyz: [f32; 3]) -> Self {
        Self { x: xyz[0], y: xyz[1], z: xyz[2] }
    }

    /// Create a face from integers
    pub const fn newi(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    pub fn newi2(x: i32, y: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: 0.
        }
    }

    pub fn empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Returns the vector defined by:
    ///    v = other - self
    /// eg the vector going from self to other
    pub fn line_to(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    pub fn equals(&self, pos: &Vector3) -> bool {
        self.x == pos[0] && self.y == pos[1] && self.z == pos[2]
    }
    
    pub fn to_cube_coordinates(&self) -> Vector3 {
        Vector3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        format!("{},{},{}", self.x, self.y, self.z).into_bytes()
    }

    pub fn from_bytes(bytes_to_parse: &[u8]) -> Self {
        let text = from_utf8(bytes_to_parse).unwrap();
        let mut pos = Vector3::empty();
        for (i, part) in text.split(',').enumerate() {
            pos[i] = part.parse::<f32>().unwrap();
        }
        pos
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_line_to() {
        let p0 = Vector3::new(0., 0., 0.);
        let p1 = Vector3::new(1., 1., 1.);
        let v01 = p0.line_to(&p1);
        assert_eq!(v01.x, 1.0);
        assert_eq!(v01.y, 1.0);
        assert_eq!(v01.z, 1.0);
        let v10 = p1.line_to(&p0);
        assert_eq!(v10.x, -1.0);
        assert_eq!(v10.y, -1.0);
        assert_eq!(v10.z, -1.0);
    }
}

/// Math operations

impl Vector3 {
    /// Dot product with another vector
    pub fn dot(&self, vec: &Vector3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(&self, vec: &Vector3) -> Self {
        Self {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x
        }
    }

    /// Returns a vector in the opposite direction
    pub fn opposite(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Returns a vector rotated 90 degrees clockwise around the z-axis
    pub fn clockwise(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    /// Returns a vector rotated 90 degrees anticlockwise around the z-axis
    pub fn anticlockwise(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }

    /// Rotation around the vertical y with angle radian
    pub fn rotation_y(&self, angle: f32) -> Self {
        Self {
            x: angle.cos() * self.x - angle.sin() * self.z,
            y: self.y,
            z: angle.sin() * self.x + angle.cos() * self.z,
        }
    }

    pub fn norm(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&mut self) {
        let n = self.norm();
        self.x /= n;
        self.y /= n;
        self.z /= n;
    }

    pub fn clamp(&mut self, min: f32, max: f32) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
        self.z = self.z.clamp(min, max);
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// impl AddAssign<Vector3> for &mut Vector3 {
//     fn add_assign(&mut self, rhs: Self) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//         self.z += rhs.z;
//     }
// }

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// impl SubAssign<Vector3> for &mut Vector3 {
//     fn sub_assign(&mut self, rhs: Self) {
//         self.x -= rhs.x;
//         self.y -= rhs.y;
//         self.z -= rhs.z;
//     }
// }

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
