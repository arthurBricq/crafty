use crate::aabb::AABB;
use crate::block_kind::Block;
use crate::primitives::vector::Vector3;

/// Model of a cube in the 3D world.
/// TODO why the fuck is there Copy here ?
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
    position: Vector3,
    block: Block,
    n_neighbors: u8,
}

impl Cube {
    pub fn new(position: [f32; 3], block: Block, neighbors: u8) -> Self {
        Self {
            position: Vector3::newf(position),
            block,
            n_neighbors: neighbors,
        }
    }

    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn block_id(&self) -> u8 {
        self.block as u8
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    pub fn neighbors_positions(position: Vector3) -> [Vector3; 6] {
        [
            position + Vector3::unit_x(),
            position - Vector3::unit_x(),
            position + Vector3::unit_y(),
            position - Vector3::unit_y(),
            position + Vector3::unit_z(),
            position - Vector3::unit_z(),
        ]
    }

    pub fn is_visible(&self) -> bool {
        self.n_neighbors < 6
    }

    pub fn add_neighhor(&mut self) {
        if self.n_neighbors < 6 {
            self.n_neighbors += 1;
        }
    }

    pub fn remove_neighbor(&mut self) {
        if self.n_neighbors > 0 {
            self.n_neighbors -= 1;
        }
    }

    pub fn set_n_neighbors(&mut self, n_neighbors: u8) {
        self.n_neighbors = n_neighbors;
    }

    pub fn n_neighbors(&self) -> u8 {
        self.n_neighbors
    }

    pub fn to_cube_coordinates(&self) -> Vector3 {
        self.position.to_cube_coordinates()
    }

    fn aabb(&self) -> AABB {
        AABB {
            north: self.position[2] + 1.,
            south: self.position[2],
            top: self.position[1] + 1.,
            bottom: self.position[1],
            east: self.position[0] + 1.,
            west: self.position[0],
        }
    }

    pub fn collides(&self, aabb: &AABB) -> bool {
        self.aabb().collides(&aabb)
    }

    pub fn collision_time(&self, aabb: &AABB, target: &AABB, velocity: &Vector3)
                          -> (f32, Vector3) {
        let cube_aabb = self.aabb();

        if aabb.collides(&cube_aabb) {
            dbg!(&aabb);
            dbg!(&cube_aabb);
            panic!("should not collide before !");
        }

        // if no collision, no need to bother
        if !target.collides(&cube_aabb) {
            return (f32::MAX, Vector3::empty());
        }

        // compute collision time in each direction
        let mut tx = if velocity[0] > 0. { (cube_aabb.west - aabb.east) / velocity[0] } else { (cube_aabb.east - aabb.west) / velocity[0] };
        let mut ty = if velocity[1] > 0. { (cube_aabb.bottom - aabb.top) / velocity[1] } else { (cube_aabb.top - aabb.bottom) / velocity[1] };

        let mut tz = if velocity[2] > 0. { (cube_aabb.south - aabb.north) / velocity[2] } else { (cube_aabb.north - aabb.south) / velocity[2] };

        // if negative, means the collision will not happen: put ∞
        if tx <= 0. { tx = f32::MAX }
        if ty <= 0. { ty = f32::MAX }
        if tz <= 0. { tz = f32::MAX }

        // collision time too big, we can discard it (for some reason, it can be
        // super high, but not ∞)
        if tx.min(ty.min(tz)) > 1e10 {
            return (f32::MAX, Vector3::empty());
        }

        if tx < ty && tx < tz {
            (tx, Vector3::unit_x() *
                if velocity[0] > 0. { -1. } else { 1. })
        } else if ty < tx && ty < tz {
            (ty, Vector3::unit_y() *
                if velocity[1] > 0. { -1. } else { 1. })
        } else if tz < tx && tz < ty {
            (tz, Vector3::unit_z() *
                if velocity[2] > 0. { -1. } else { 1. })
        } else {
            dbg!(tx, ty, tz);
            panic!("should not be here:");
        }
    }
}
