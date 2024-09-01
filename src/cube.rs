use crate::block_kind::Block;
use crate::vector::Vector3;
use crate::aabb::{AABB, DisplacementStatus};

/// Model of a cube in the 3D world.
#[derive(Clone, Copy, Debug)]
pub struct Cube {
    position: Vector3,
    block: Block,
    is_visible: bool
}

impl Cube {
    pub fn new(position: [f32; 3], block: Block, visible: bool) -> Self {
        Self { position: Vector3::newf(position), block, is_visible: visible }
    }

    pub fn model_matrix(&self) -> [[f32; 4]; 4] {
        // TODO As you can see, I added 0.5 at each cube model
        //      It's because I was lazy to edit all the values in `VERTICES` of +0.5, but
        //      it would be nice to do it eventually :)
        [
            [1.00, 0.0, 0.0, 0.0],
            [0.0, 1.00, 0.0, 0.0],
            [0.0, 0.0, 1.00, 0.0],
            [self.position[0] + 0.5, self.position[1] + 0.5, self.position[2] + 0.5, 1.0f32]
        ]
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
    
    pub fn neighbors_positions(&self) -> [Vector3; 6] {
        [
            self.position + Vector3::unit_x(),
            self.position - Vector3::unit_x(),
            self.position + Vector3::unit_y(),
            self.position - Vector3::unit_y(),
            self.position + Vector3::unit_z(),
            self.position - Vector3::unit_z(),
        ]
    }

    pub fn set_is_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
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
	if self.aabb().collides(&aabb) {
	    dbg!("collides with cube");
	    dbg!(self);
	    dbg!(aabb);

	    return true
	}
	false
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
	    return (f32::MAX, Vector3::empty())
	}
	
	dbg!("previous collision was target collision check");
	
	// compute collision time in each direction
	let mut tx = if velocity[0] > 0. { (cube_aabb.west - aabb.east) / velocity[0]}
	else { (cube_aabb.east - aabb.west) / velocity[0] };
	let mut ty = if velocity[1] > 0. { (cube_aabb.bottom - aabb.top) / velocity[1]}
	else { (cube_aabb.top - aabb.bottom) / velocity[1] };
	
	let mut tz = if velocity[2] > 0. { (cube_aabb.south - aabb.north) / velocity[2]}
	else { (cube_aabb.north - aabb.south) / velocity[2] };

	// if negative, means the collision will not happen: put ∞
	if tx <= 0. { tx = f32::MAX }
	if ty <= 0. { ty = f32::MAX }
	if tz <= 0. { tz = f32::MAX }

	// tx.min(ty.min(tz))

	// collision time too big, we can discard it (for some reason, it can be
	// super high, but not ∞)
	if tx.min(ty.min(tz)) > 1e10 {
	    return (f32::MAX, Vector3::empty())
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
