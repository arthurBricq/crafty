use crate::primitives::vector::Vector3;
use std::fmt::{Display, Error, Formatter};
use crate::primitives::face::Plane3;

#[derive(Debug)]
pub struct AABB {
    north: f32,
    south: f32,
    top: f32,
    bottom: f32,
    east: f32,
    west: f32,
}

#[derive(Debug)]
pub enum AABBError {
    WrongXRange,
    WrongYRange,
    WrongZRange
}

impl Display for AABBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self {
            AABBError::WrongXRange => "WrongXRange",
            AABBError::WrongYRange => "WrongYRange",
            AABBError::WrongZRange => "WrongZRange"
        })
    }
}

impl AABB {
    pub fn new(north: f32, south: f32, top: f32, bottom: f32,
               east: f32, west: f32) -> Result<Self, AABBError> {
        if north <= south { return Err(AABBError::WrongZRange) }
        if top <= bottom { return Err(AABBError::WrongYRange) }
        if east <= west { return Err(AABBError::WrongXRange) }

        Ok(AABB { north, south, top, bottom, east, west })
    }

    pub fn north(&self) -> f32 { self.north }
    pub fn south(&self) -> f32 { self.south }
    pub fn top(&self) -> f32 { self.top }
    pub fn bottom(&self) -> f32 { self.bottom }
    pub fn east(&self) -> f32 { self.east }
    pub fn west(&self) -> f32 { self.west }
    
    pub fn collides(&self, aabb: &AABB) -> bool {
        self.west <= aabb.east &&
            self.east >= aabb.west &&
            self.bottom <= aabb.top &&
            self.top >= aabb.bottom &&
            self.south <= aabb.north &&
            self.north >= aabb.south
    }
    
    pub fn faces(&self) -> [Plane3; 6] {
        let bottom_left = Vector3::new(self.west, self.bottom, self.south);
        let unit_x = Vector3::unit_x() * (self.east - self.west);
        let unit_y = Vector3::unit_x() * (self.top - self.bottom);
        let unit_z = Vector3::unit_x() * (self.north - self.south);
        [
            Plane3::new(bottom_left, unit_x, unit_y, Vector3::empty()),
            Plane3::new(bottom_left, unit_y, unit_z, Vector3::empty()),
            Plane3::new(bottom_left, unit_z, unit_x, Vector3::empty()),
            Plane3::new(bottom_left + unit_z, unit_x, unit_y, Vector3::empty()),
            Plane3::new(bottom_left + unit_x, unit_z, unit_y, Vector3::empty()),
            Plane3::new(bottom_left + unit_y, unit_x, unit_z, Vector3::empty()),
        ]
    }
    
}

#[cfg(test)]
mod tests {
    use crate::aabb::*;

    #[test]
    fn test_aabb_collision() {
        let aabb = AABB {
            north: 3.,
            south: 2.,
            top: 2.,
            bottom: 1.,
            east: 4.,
            west: 3.,
        };

        let aabb2 = AABB {
            north: 3.25,
            south: 2.75,
            top: 2.832886,
            bottom: 1.032886,
            east: 4.25,
            west: 3.75,
        };

        assert!(aabb.collides(&aabb2));
    }
}
