use crate::vector::Vector3;

/// An action is something that will alter the world
pub enum Action {
    /// Destroys a cube of the world
    Destroy {
        at: Vector3
    }
}