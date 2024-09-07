use crate::primitives::matrix::Matrix3;
use crate::primitives::vector::Vector3;

/// A 3D plane, spawning from a point and 2 vectors
pub struct Plane3 {
    origin: Vector3,
    u: Vector3,
    v: Vector3,
    /// Vector that sends `origin` to the `origin` of the adjacent cube
    to_adjacent_cube: Vector3
}

impl Plane3 {
    pub fn new(origin: Vector3, u: Vector3, v: Vector3, to_adjacent_cube: Vector3) -> Self {
        Self { origin, u, v, to_adjacent_cube}
    }
    
    /// Computes the intersection between Self and the ray defined by the position and direction.
    /// If there is one, returns the distance between the player and Self.
    /// Otherwise returns None
    /// 
    /// If there is an intersection, then it holds true that
    ///        camera + t * direction = origin + alpha * u + beta * v
    /// Therefore
    ///       [alpha, beta, -t] = [u | v | -direction].inv() * (camera - origin) 
    pub fn face_intersection(&self, camera: Vector3, direction: Vector3) -> Option<f32> {
        Matrix3::from_columns(&self.u, &self.v, &direction)
            .linear_solve(camera - self.origin)
            .filter(|res| res.x() >= 0. && res.x() <= 1.0 &&  res.y() >= 0. && res.y() <= 1.0 && res.z() < 0.)
            .map(|res| -res.z())
    }
    
    /// Returns the position of the cube adjacent to this plane
    pub fn adjacent_cube(&self) -> Vector3 {
        self.origin + self.to_adjacent_cube
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::face::Plane3;
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_face_intersection() {
        let P  =  Plane3::new(Vector3::empty(), Vector3::unit_x(), Vector3::unit_y(), Vector3::empty());
        
        // When looking toward the face, there is an intersection
        assert_eq!(Some(10.), P.face_intersection(Vector3::new(0.5, 0.5, 10.), Vector3::unit_z().opposite()));
        
        // When looking somewhere else, no intersection
        assert_eq!(None, P.face_intersection(Vector3::new(0.5, 0.5, 10.), Vector3::unit_z()));
        assert_eq!(None, P.face_intersection(Vector3::new(0.5, 0.5, 10.), Vector3::unit_y()));
        assert_eq!(None, P.face_intersection(Vector3::new(0.5, 0.5, 10.), Vector3::unit_x()));
        
        // Make sure that only points inside the [0;1] face are valid
        assert_eq!(Some(10.), P.face_intersection(Vector3::new(0.2, 0.1, 10.), Vector3::unit_z().opposite()));
        assert_eq!(Some(10.), P.face_intersection(Vector3::new(0.2, 0.9, 10.), Vector3::unit_z().opposite()));
        assert_eq!(Some(10.), P.face_intersection(Vector3::new(0.001, 0.999, 10.), Vector3::unit_z().opposite()));
        assert_eq!(None, P.face_intersection(Vector3::new(0.001, 1.999, 10.), Vector3::unit_z().opposite()));
        assert_eq!(None, P.face_intersection(Vector3::new(1.001, 0.5, 10.), Vector3::unit_z().opposite()));
        
    }
    
}