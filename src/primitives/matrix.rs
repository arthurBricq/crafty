use crate::primitives::vector::Vector3;
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Matrix3 {
    a11: f32,
    a12: f32,
    a13: f32,
    a21: f32,
    a22: f32,
    a23: f32,
    a31: f32,
    a32: f32,
    a33: f32,
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(
            self.a11 * rhs.x() + self.a12 * rhs.y() + self.a13 * rhs.z(),
            self.a21 * rhs.x() + self.a22 * rhs.y() + self.a23 * rhs.z(),
            self.a31 * rhs.x() + self.a32 * rhs.y() + self.a33 * rhs.z(),
        )
    }
}

impl Mul<f32> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix3 {
            a11: self.a11 * rhs,
            a12: self.a12 * rhs,
            a13: self.a13 * rhs,
            a21: self.a21 * rhs,
            a22: self.a22 * rhs,
            a23: self.a23 * rhs,
            a31: self.a31 * rhs,
            a32: self.a32 * rhs,
            a33: self.a33 * rhs,
        }
    }
}

impl Matrix3 {
    /// Create a matrix from its 3 rows
    pub fn from_columns(c1: &Vector3, c2: &Vector3, c3: &Vector3) -> Self {
        Self {
            a11: c1[0],
            a21: c1[1],
            a31: c1[2],
            a12: c2[0],
            a22: c2[1],
            a32: c2[2],
            a13: c3[0],
            a23: c3[1],
            a33: c3[2],
        }
    }

    #[cfg(test)]
    pub fn identity() -> Self {
        Self {
            a11: 1.0,
            a21: 0.0,
            a31: 0.0,
            a12: 0.0,
            a22: 1.0,
            a32: 0.0,
            a13: 0.0,
            a23: 0.0,
            a33: 1.0,
        }
    }

    /// Solve the linear equation
    /// A x = rhs
    /// If there are no solution, returns none
    pub fn linear_solve(&self, rhs: Vector3) -> Option<Vector3> {
        if self.invertible() {
            Some(self.inverse() * rhs)
        } else {
            None
        }
    }
}

impl Matrix3 {
    fn determinant(&self) -> f32 {
        self.a11 * self.a22 * self.a33
            + self.a12 * self.a23 * self.a31
            + self.a13 * self.a21 * self.a32
            - self.a13 * self.a22 * self.a31
            - self.a12 * self.a21 * self.a33
            - self.a11 * self.a23 * self.a32
    }

    fn invertible(&self) -> bool {
        self.determinant() != 0.
    }

    fn inverse(&self) -> Matrix3 {
        // reference : https://semath.info/src/inverse-cofactor-ex3.html
        let a11 = self.a11;
        let a22 = self.a22;
        let a33 = self.a33;
        let a12 = self.a12;
        let a13 = self.a13;
        let a21 = self.a21;
        let a23 = self.a23;
        let a31 = self.a31;
        let a32 = self.a32;

        let inv = 1. / self.determinant();

        Matrix3 {
            a11: (a22 * a33 - a23 * a32) * inv,
            a12: -(a12 * a33 - a13 * a32) * inv,
            a13: (a12 * a23 - a13 * a22) * inv,
            a21: -(a21 * a33 - a23 * a31) * inv,
            a22: (a11 * a33 - a13 * a31) * inv,
            a23: -(a11 * a23 - a13 * a21) * inv,
            a31: (a21 * a32 - a22 * a31) * inv,
            a32: -(a11 * a32 - a12 * a31) * inv,
            a33: (a11 * a22 - a12 * a21) * inv,
        }
    }

    #[cfg(test)]
    fn col(&self, i: usize) -> Vector3 {
        match i {
            0 => Vector3::new(self.a11, self.a21, self.a31),
            1 => Vector3::new(self.a12, self.a22, self.a32),
            2 => Vector3::new(self.a13, self.a23, self.a33),
            _ => panic!("Not possible"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::matrix::Matrix3;
    use crate::primitives::vector::Vector3;

    const EPSILON: f32 = 0.0001;

    pub fn assert_near(left: Vector3, right: Vector3) {
        println!("{left:?} vs {right:?}");
        assert!(f32::abs(left.x() - right.x()) < EPSILON);
        assert!(f32::abs(left.y() - right.y()) < EPSILON);
        assert!(f32::abs(left.z() - right.z()) < EPSILON);
    }

    #[test]
    fn inverse_simple_matrix() {
        let m1 = Matrix3::identity();
        let m2 = m1.inverse();
        assert_near(Vector3::new(1., 0., 0.), m2.col(0));
        assert_near(Vector3::new(0., 1., 0.), m2.col(1));
        assert_near(Vector3::new(0., 0., 1.), m2.col(2));

        let m3 = Matrix3::identity() * 2.;
        let m4 = m3.inverse();
        assert_near(Vector3::new(0.5, 0., 0.), m4.col(0));
        assert_near(Vector3::new(0., 0.5, 0.), m4.col(1));
        assert_near(Vector3::new(0., 0., 0.5), m4.col(2));
    }
}
