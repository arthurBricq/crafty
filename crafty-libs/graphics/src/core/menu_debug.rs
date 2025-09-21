use std::f32::consts::PI;
use primitives::position::Position;
use crate::core::rectangle::RectInstance;
use crate::core::string_rect::StringRect;

/// Data in the debug menu
pub const DEBUG_MENU_DATA: [DebugItem; 4] = [
    DebugItem::new("fps:"),
    DebugItem::new("coord:"),
    DebugItem::new("rot:"),
    DebugItem::new("cube rendered:"),
];

pub struct DebugData {
    fps: f32,
    pos: Position,
    cube_rendered: usize,
}

impl DebugData {
    pub fn new(fps: f32, pos: Position, cube_rendered: usize) -> Self {
        Self {
            fps,
            pos: pos.clone(),
            cube_rendered,
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn cube_rendered(&self) -> usize {
        self.cube_rendered
    }
}

/// Item for Debug menu: contain a string to be displayed
#[derive(Clone, Copy)]
pub struct DebugItem {
    element: &'static str,
}

impl DebugItem {
    pub const fn new(element: &'static str) -> Self {
        Self { element }
    }

    pub fn element(&self) -> &str {
        &self.element
    }
}

pub struct DebugMenuData {
    items: Vec<DebugItem>,
}

impl DebugMenuData {
    pub fn new(items: Vec<DebugItem>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &Vec<DebugItem> {
        &self.items
    }
}

/// Implement the debug menu
pub struct DebugMenu {
    static_part: Vec<RectInstance>,
    coord_to_update: Vec<[f32; 2]>,
    rects: Vec<RectInstance>,
}

impl DebugMenu {
    pub fn new(debug_menu_data: &DebugMenuData) -> Self {
        let mut static_part = Vec::new();
        let mut coord_to_update: Vec<[f32; 2]> = Vec::new();
        let mut y = 0.8;
        let u = -0.95;
        let size = 0.015;
        StringRect::write_string_centered(
            y + 8. * size,
            1.8 * size,
            &String::from("debug menu"),
            &mut static_part,
        );

        for item in debug_menu_data.items() {
            StringRect::write_string(u, y, size, &item.element().to_string(), &mut static_part);
            coord_to_update.push([u + 0.4, y]);
            y -= 4. * size;
        }
        Self {
            rects: static_part.clone(),
            static_part,
            coord_to_update,
        }
    }

    pub fn rects(&self) -> &Vec<RectInstance> {
        &self.rects
    }

    /// Create the new text rectangle from the DebugData
    pub fn set_items(&mut self, debug_data: DebugData) {
        self.rects = self.static_part.clone();

        let fps_string = &format!("{:5.1}", debug_data.fps());
        StringRect::write_string(
            self.coord_to_update[0][0],
            self.coord_to_update[0][1],
            0.015,
            fps_string,
            &mut self.rects,
        );

        let pos_string = &format!(
            "{:7.3}:{:7.3}:{:7.3}",
            debug_data.pos().x(),
            debug_data.pos().y(),
            debug_data.pos().z()
        );
        StringRect::write_string(
            self.coord_to_update[1][0],
            self.coord_to_update[1][1],
            0.015,
            pos_string,
            &mut self.rects,
        );

        let rot_string = &format!(
            "{:7.3}:{:7.3}",
            debug_data.pos().yaw() % (2. * PI),
            debug_data.pos.pitch()
        );
        StringRect::write_string(
            self.coord_to_update[2][0],
            self.coord_to_update[2][1],
            0.015,
            rot_string,
            &mut self.rects,
        );

        let cube_string = &format!("{:7}", debug_data.cube_rendered());
        StringRect::write_string(
            self.coord_to_update[3][0] + 0.3,
            self.coord_to_update[3][1],
            0.015,
            cube_string,
            &mut self.rects,
        );
    }
}
