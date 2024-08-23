use crate::graphics::rectangle::RectVertexAttr;
use crate::graphics::string_rect::StringRect;
use crate::vector::Vector3;

use std::f32::consts::PI;

/// Data in the debug menu
pub const DEBUG_MENU_DATA: [DebugItem;3] = [

    DebugItem::new("fps:"),
    DebugItem::new("coord:"),
    DebugItem::new("rot:"),
];


pub struct DebugData {
    fps: f32,
    pos: Vector3,
    rotation: [f32;2]
}

impl DebugData {
    pub fn new(fps: f32, pos: Vector3, rotation: [f32;2]) -> Self {
        Self { fps, pos: pos.clone(), rotation }
    }
    
    pub fn fps(&self) -> f32 {
        self.fps
    }
    
    pub fn pos(&self) -> Vector3 {
        self.pos
    }
    
    pub fn rotation(&self) -> [f32; 2] {
        self.rotation
    }
}

/// Item for Debug menu: contain a string to be displayed
#[derive(Clone,Copy)]
pub struct DebugItem {
    element: &'static str
}

impl DebugItem {
    pub const fn new(element: &'static str) -> Self {
        Self {
            element
        }
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
        Self {
            items
        }
    }

    pub fn items(&self) -> &Vec<DebugItem> {
        &self.items
    }

}

/// Implement the debug menu
pub struct DebugMenu {
    static_part:Vec<RectVertexAttr>,
    dynamic_part:Vec<Vec<RectVertexAttr>>,
    coord_to_update: Vec<[f32;2]>,
    rects: Vec<RectVertexAttr>
}

impl DebugMenu {
    pub fn new(debug_menu_data: &DebugMenuData) -> Self {
        let mut static_part= Vec::new();
        let mut dynamic_part: Vec<Vec<RectVertexAttr>>= Vec::new();
        let mut coord_to_update: Vec<[f32;2]> = Vec::new();
        let mut y= 0.8;
        let u= -0.95;
        let size = 0.015;
        StringRect::write_string_centered( y+8. * size, 1.8 * size, &String::from("debug menu"), &mut static_part);

        for item in debug_menu_data.items() {
            StringRect::write_string(u, y, size, &item.element().to_string(), &mut static_part);
            dynamic_part.push(Vec::new());
            coord_to_update.push([u+0.4,y]);
            y-=4. * size;
        }
        Self {
            rects: static_part.clone(),
            static_part,
            dynamic_part,
            coord_to_update
        }
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }

    /// Create the new text rectangle from the DebugData
    pub fn set_items(&mut self, debug_data: DebugData)
    {
        self.rects = self.static_part.clone();

        let fps_string = &format!("{:5.1}",debug_data.fps());
        StringRect::write_string(self.coord_to_update[0][0], self.coord_to_update[0][1], 0.015, fps_string, &mut self.rects);
        
        let pos_string = &format!("{:7.3}:{:7.3}:{:7.3}",debug_data.pos().x(),debug_data.pos().y(),debug_data.pos().z());
        StringRect::write_string(self.coord_to_update[1][0], self.coord_to_update[1][1], 0.015, pos_string, &mut self.rects);
        
        let rot_string = &format!("{:7.3}:{:7.3}",debug_data.rotation()[0] % (2.*PI),debug_data.rotation()[1]);
        StringRect::write_string(self.coord_to_update[2][0], self.coord_to_update[2][1], 0.015, rot_string, &mut self.rects);
    }
}

