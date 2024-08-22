use crate::vector::Vector3;

#[derive(Debug)]
pub struct AABB {
    pub north: f32,
    pub south: f32,
    pub top: f32,
    pub bottom: f32,
    pub east: f32,
    pub west: f32,
}

#[derive(Debug)]
pub enum DisplacementStatus {
    Still, Forward, Backward
}

impl DisplacementStatus {
    pub fn from_scalar(displacement: f32) -> DisplacementStatus {
	if displacement < 0.
	{ Self::Backward }
	else if displacement > 0.
	{ Self::Forward }
	else
	{ Self::Still }
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