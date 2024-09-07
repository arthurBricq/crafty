use crate::aabb::AABB;
use crate::primitives::vector::Vector3;

/// Informations about a collision
pub struct CollisionData {
    /// Time till the collision, in s
    pub time: f32, 
    /// Normal of the colliding surface, at the point of collision
    pub normal: Vector3, 
}

/// Trait to define static (i.e. that do not move) objects that the entities can
/// collide with (i.e. cubes, or compound objects like chunks or worlds)
pub trait Collidable {
    /// Simply check the collision between this and the given AABB
    fn collides(&self, aabb: &AABB) -> bool;

    /// Assuming the `aabb` is moving at a constant velocity `velocity`,
    /// computes the time it will take for it to collide. If no collision will
    /// occur, returns None. `target` contains the final position, useful to
    /// shortcut the calculation.
    ///
    /// Note that the implementor of this trait can assume that `aabb` is not
    /// colliding already, and can panic if it does.
    fn collision_time(&self, aabb: &AABB, target: &AABB, velocity: &Vector3)
                          -> Option<CollisionData>;

}
