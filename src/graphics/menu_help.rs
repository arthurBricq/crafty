use crate::graphics::rectangle::RectVertexAttr;
use crate::graphics::font::GLChar;
use crate::graphics::color::Color::LightGray;
use crate::graphics::color::Color::LightCoral;
use crate::graphics::string_rect::StringRect;

pub struct HelpMenuData {
    items: Vec<StringRect>,
    y: f32,
    size: f32
}

impl HelpMenuData {
    pub fn new() -> Self {
        HelpMenuData {
            items:Vec::new(),
            y: 0.6,
            size: 0.025
        }
    }
    
    /// Add two text on the same line
    pub fn add_line(&mut self, string: String, string2: String) {
        self.add_item(string, -0.9);
        self.add_item_end_line(string2, 0.5);
    }

    pub fn add_item(&mut self, effect:String, u:f32) {
        self.items.push(StringRect::new(effect, u, self.y, self.size));
    }

    pub fn add_item_end_line(&mut self, effect:String, u:f32) {
        self.items.push(StringRect::new(effect, u, self.y, self.size));
        self.y-=4. * self.size;
    }

    pub fn items(&self) -> &Vec<StringRect> {
        &self.items
    }

}

/// Implement the help menu
pub struct HelpMenu {
    rects:Vec<RectVertexAttr>,
    help_menu_data: HelpMenuData
}

impl HelpMenu {
    pub fn new() -> Self {
        let mut help_menu = HelpMenu {
            rects: Vec::new(),
            help_menu_data: HelpMenuData::new()
        };

        help_menu.help_menu_data.add_line(String::from("move forward"),String::from("z"));
        help_menu.help_menu_data.add_line(String::from("move backward"),String::from("s"));
        help_menu.help_menu_data.add_line(String::from("move left"),String::from("q"));
        help_menu.help_menu_data.add_line(String::from("move right"),String::from("d"));
        help_menu.help_menu_data.add_line(String::from("jump"),String::from("space"));
        help_menu.help_menu_data.add_line(String::from("help menu"),String::from("f12"));
        help_menu.update();

        help_menu
    }

    /// Recreate vectors of rectangle from the list of items
    pub fn update(&mut self) {
        self.rects=Vec::new();
        self.rects.push(RectVertexAttr::new_from_corner(-0.95, 0.70, 1.8, -0.7, LightCoral));
        for item in self.help_menu_data.items() {
            self.rects.append(&mut item.rects().clone()); 
        }
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }
}