use winit::event::ElementState;

pub enum InventoryEvent {
    /// Contains the x, y coordinates in NDC of the new position
    CursorMoved(f32, f32),
    /// Only left click for now
    Button(ElementState),
}
