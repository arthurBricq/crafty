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

    pub fn collision(&self, aabb: &AABB, displacement_status: &[DisplacementStatus; 3]) -> Vector3 {
	let mut signature = Vector3::empty();

	// first, need to make sure we collide along any axis
	if  aabb.east < self.position[0] ||
	    aabb.west > self.position[0] + 1. ||

	    // TODO I've put >=, but idk if it's good or not
	    aabb.top < self.position[1] ||
	    aabb.bottom >= self.position[1] + 1. ||
	    
	    aabb.north < self.position[2] ||
	    aabb.south > self.position[2] + 1. {
		signature[0] = f32::MAX;
		signature[1] = f32::MAX;
		signature[2] = f32::MAX;

		return signature
	    }
	    
	

	// if not, don't bother computing any signature
	

	signature[0] = match displacement_status[0] {
	    DisplacementStatus::Forward => if aabb.west > self.position[0] + 1. { f32::MAX }
	    else { self.position[0] - aabb.east },
	    // DisplacementStatus::Forward => self.position[0] - aabb.east,
	    DisplacementStatus::Backward => if aabb.east < self.position[0] { f32::MAX }
	    else { aabb.west - self.position[0] - 1. },
	    DisplacementStatus::Still => f32::MAX
	};

	signature[1] = match displacement_status[1] {
	    DisplacementStatus::Forward => if aabb.bottom > self.position[1] + 1. { f32::MAX }
	    else { self.position[1] - aabb.top },
	    // DisplacementStatus::Forward => self.position[1] - aabb.top,
	    DisplacementStatus::Backward => if aabb.top < self.position[1] { f32::MAX }
	    else { aabb.bottom - self.position[1] - 1. },
	    // DisplacementStatus::Backward => aabb.bottom - self.position[1] - 1.,
	    DisplacementStatus::Still => f32::MAX
	};

	signature[2] = match displacement_status[2] {
	    DisplacementStatus::Forward => if aabb.south > self.position[2] + 1. { f32::MAX }
	    else { self.position[2] - aabb.north },
	    // DisplacementStatus::Forward => self.position[2] - aabb.north,
	    DisplacementStatus::Backward => if aabb.north < self.position[2] { f32::MAX }
	    else { aabb.south - self.position[2] - 1. },
	    // DisplacementStatus::Backward => aabb.south - self.position[2] - 1.,
	    DisplacementStatus::Still => f32::MAX
	};

	if signature[0] < 0. || signature[2] < 0. {
	    dbg!("cube colliding in plane !");
	    dbg!(signature);
	    dbg!(displacement_status);
	    dbg!(self);
	    dbg!(&aabb);
	    dbg!([(aabb.north + aabb.south) /  2.,
		  (aabb.east + aabb.west) /  2.]);
	}
	
	signature
    }
}
