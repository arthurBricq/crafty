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
