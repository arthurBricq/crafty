use crate::graphics::rectangle::RectVertexAttr;
use crate::graphics::font::GLChar;
use crate::graphics::color::Color::LightGray;
use crate::graphics::color::Color::LightCoral;
use crate::graphics::string_rect::StringRect;


pub struct HelpMenuItem {
    command: String,
    key: String
}

impl HelpMenuItem {

    pub fn new(command: String, key: String) -> Self {
        Self {
            command,
            key
        }
    }
    pub fn command(&self) -> &String {
        &self.command        
    }
    pub fn key(&self) -> &String {
        &self.key
    }
}

pub struct HelpMenuData {
    items: Vec<HelpMenuItem>,
}

impl HelpMenuData {
    pub fn new(items: Vec<HelpMenuItem>) -> Self {
        Self {
            items
        }
    }
    
    pub fn add_item(&mut self, command: String, key: String) {
        self.items.push(HelpMenuItem::new(command, key))
    }

    pub fn items(&self) -> &Vec<HelpMenuItem> {
        &self.items
    }

}

/// Implement the help menu
pub struct HelpMenu {
    rects:Vec<RectVertexAttr>,
}

impl HelpMenu {
    pub fn new(help_menu_data: &HelpMenuData) -> Self {
        let mut rects= Vec::new();
        let mut y= 0.6;
        let u= -0.9;
        let size = 0.025;
        for item in help_menu_data.items() {
            for rect in StringRect::new(item.command(), u, y, size).rects() {
            rects.push(*rect);
            }
            for rect in StringRect::new(item.key(), u+0.9, y, size).rects() {
                rects.push(*rect);
            }
            y-=4. * size;
        }
        Self {
            rects
        }
    }


    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }
}