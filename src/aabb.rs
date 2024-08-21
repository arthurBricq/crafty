use crate::primitives::vector::Vector3;

#[derive(Debug)]
pub struct AABB {
    pub north: f32,
    pub south: f32,
    pub top: f32,
    pub bottom: f32,
    pub east: f32,
    pub west: f32,
}

impl AABB {
    pub fn collides(&self, aabb: &AABB) -> bool {
        self.west <= aabb.east &&
            self.east >= aabb.west &&

            self.bottom <= aabb.top &&
            self.top >= aabb.bottom &&

            self.south <= aabb.north &&
            self.north >= aabb.south
    }
}

#[derive(Debug)]
pub enum DisplacementStatus {
    Still,
    Forward,
    Backward,
}

impl DisplacementStatus {
    pub fn from_scalar(displacement: f32) -> DisplacementStatus {
        if displacement < 0.
        { Self::Backward } else if displacement > 0.
        { Self::Forward } else { Self::Still }
    }

    pub fn to_scalar(&self) -> f32 {
        match self {
            Self::Still => 0.,
            Self::Forward => 1.,
            Self::Backward => -1.,
        }
    }

    pub fn from_vector(displacement: Vector3) -> [DisplacementStatus; 3] {
        [
            Self::from_scalar(displacement[0]),
            Self::from_scalar(displacement[1]),
            Self::from_scalar(displacement[2])
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
