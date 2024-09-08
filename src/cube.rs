use crate::aabb::AABB;
use crate::block_kind::Block;
use crate::collidable::{Collidable, CollisionData};
use crate::primitives::face::Plane3;
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

    pub fn is_transparent(&self) -> bool {
        self.block.is_transparent()
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

    /// Returns the faces of this cube.
    /// Note that each face points to a neighbor cube using the `to_adjacent_cube`
    pub fn faces(&self) -> [Plane3; 6] {
        [
            Plane3::new(self.position, Vector3::unit_x(), Vector3::unit_y(), Vector3::unit_z().opposite()),
            Plane3::new(self.position, Vector3::unit_y(), Vector3::unit_z(), Vector3::unit_x().opposite()),
            Plane3::new(self.position, Vector3::unit_z(), Vector3::unit_x(), Vector3::unit_y().opposite()),
            Plane3::new(self.position + Vector3::unit_z(), Vector3::unit_x(), Vector3::unit_y(), Vector3::empty()),
            Plane3::new(self.position + Vector3::unit_x(), Vector3::unit_z(), Vector3::unit_y(), Vector3::empty()),
            Plane3::new(self.position + Vector3::unit_y(), Vector3::unit_x(), Vector3::unit_z(), Vector3::empty()),
        ]
    }

    /// Computes the intersection between a ray and 6 faces
    /// Returns (distance, index in the array of faces)
    pub fn intersection_with_faces(faces: &[Plane3; 6], pos: Vector3, dir: Vector3,) -> Option<(f32, usize)> {
        let mut best_result: Option<(f32, usize)> = None;
        for i in 0..faces.len() {
            if let Some(t) = faces[i].face_intersection(pos, dir) {
                if best_result.is_none() || t < best_result.unwrap().0 {
                    best_result = Some((t, i));
                }
            }
        }
        best_result
    }
    
    /// Returns the distance between this cube and the the player, computed from ray-tracing
    pub fn intersection_with(&self, pos: Vector3, dir: Vector3) -> Option<f32> {
        Self::intersection_with_faces(&self.faces(), pos, dir).map(|(dist, _)| dist)
    }

    /// Returns the position where to add a new cube, given
    /// `touched_cube`    : ref to the cube being touched
    /// `camera_pos`      : position of the player
    /// `camera_dir`      : direction of the player
    pub fn position_to_add_new_cube(&self, pos: Vector3, dir: Vector3) -> Result<Vector3, u8> {
        let index = Self::intersection_with_faces(&self.faces(), pos, dir)
            .map(|(_, index)| index).ok_or(0)?;
        Ok(self.faces()[index].adjacent_cube())
    }

    pub fn cube_aabb(pos: Vector3) -> AABB {
        AABB::new(
            pos[2] + 1.,
            pos[2],
            pos[1] + 1.,
            pos[1],
            pos[0] + 1.,
            pos[0],
        ).unwrap()
    }

    fn aabb(&self) -> AABB {
        Cube::cube_aabb(self.position)
    }
}

impl Collidable for Cube {
    fn collides(&self, aabb: &AABB) -> bool {
        self.aabb().collides(&aabb)
    }

    fn collision_time(
        &self,
        aabb: &AABB,
        target: &AABB,
        velocity: &Vector3,
    ) -> Option<CollisionData> {
        let cube_aabb = self.aabb();

        if aabb.collides(&cube_aabb) {
            dbg!(&aabb);
            dbg!(&cube_aabb);

            panic!("should not collide before !");
        }

        // if no collision, no need to bother
        if !target.collides(&cube_aabb) {
            return None;
        }

        // compute collision time in each direction
        let mut tx = if velocity[0] > 0. {
            (cube_aabb.west() - aabb.east()) / velocity[0]
        } else {
            (cube_aabb.east() - aabb.west()) / velocity[0]
        };
        let mut ty = if velocity[1] > 0. {
            (cube_aabb.bottom() - aabb.top()) / velocity[1]
        } else {
            (cube_aabb.top() - aabb.bottom()) / velocity[1]
        };

        let mut tz = if velocity[2] > 0. {
            (cube_aabb.south() - aabb.north()) / velocity[2]
        } else {
            (cube_aabb.north() - aabb.south()) / velocity[2]
        };

        // if negative, means the collision will not happen: put ∞
        if tx <= 0. {
            tx = f32::MAX
        }
        if ty <= 0. {
            ty = f32::MAX
        }
        if tz <= 0. {
            tz = f32::MAX
        }

        // collision time too big, we can discard it (for some reason, it can be
        // super high, but not ∞)
        if tx.min(ty.min(tz)) > 1e10 {
            return None;
        }

        if tx < ty && tx < tz {
            Some(CollisionData {
                time: tx,
                normal: Vector3::unit_x() * if velocity[0] > 0. { -1. } else { 1. },
            })
        } else if ty < tx && ty < tz {
            Some(CollisionData {
                time: ty,
                normal: Vector3::unit_y() * if velocity[1] > 0. { -1. } else { 1. },
            })
        } else if tz < tx && tz < ty {
            Some(CollisionData {
                time: tz,
                normal: Vector3::unit_z() * if velocity[2] > 0. { -1. } else { 1. },
            })
        } else {
            dbg!(tx, ty, tz);
            panic!("should not be here:");
        }
    }
}
