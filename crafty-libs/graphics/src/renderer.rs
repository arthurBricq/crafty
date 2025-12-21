use std::time::Duration;

// Re-export render data types from primitives for convenience
pub use primitives::render_data::{CubeRenderData, EntityRenderData, RectRenderData};

/// Trait that OpenGL frontends must implement.
pub trait Renderer: Default {
    // fn new_with_backend(backend: impl RendererBackend) -> Self;

    // TODO: add an `initialize` method
    // fn run(&mut self);
    fn run<B: RendererBackend>(&self, backend: &mut B);
}

pub trait RendererBackend {
    fn update(&mut self, dt: Duration) -> ToDraw;
    fn set_dimension(&mut self, dimension: (u32, u32));
    fn handle_mouse_event(&mut self, event: MouseEvent);
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Vec<WindowAction>;
    fn cursor_moved(&mut self, x: f32, y: f32);
    fn handle_motion_event(&mut self, axis: u32, value: f64);
}

pub struct ToDraw {
    pub player_view_matrix: [[f32; 4]; 4],
    pub selected_intensity: f32,
    // TODO: for performances, we should consider to have a borrowed type here.
    //       Having an owned value (Vec) was introduced when improving dependencies related to glium
    pub cubes_buffer: Vec<CubeRenderData>,
    pub entity_buffer: Vec<EntityRenderData>,
    pub hud_buffer: Vec<RectRenderData>,
}

pub enum WindowAction {
    SetFullscreen(bool),
    SetCursor(bool),
}

// TODO: these types could probably be moved to another file, idk
//       Maybe actually it should be the `renderer` to be moved to another crate ?

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Other,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PressedOrReleased {
    Pressed,
    Released,
}

impl PressedOrReleased {
    pub const fn is_pressed(&self) -> bool {
        match self {
            PressedOrReleased::Pressed => true,
            PressedOrReleased::Released => false,
        }
    }
}

pub struct MouseEvent {
    pub button: MouseButton,
    pub state: PressedOrReleased,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Escape,
    KeyE,
    KeyW,
    KeyS,
    KeyD,
    KeyA,
    KeyK,
    KeyJ,
    Space,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    KeyP,
    KeyX,
    F3,
    F10,
    F11,
    F12,
    None,
}

pub struct KeyEvent {
    pub key: KeyCode,
    pub state: PressedOrReleased,
}
