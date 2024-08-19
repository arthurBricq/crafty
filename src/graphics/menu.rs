use crate::graphics::rectangle::RectVertexAttr;
use crate::graphics::font::GLChar;
use crate::graphics::color::Color::LightGray;
use crate::graphics::color::Color::LightCoral;

// One line of the help menu
pub struct HelpItem {
    effect: String,
    rects: Vec<RectVertexAttr>,
    u: f32,
    v: f32,
    size: f32
}

impl HelpItem {
    pub fn new(effect: String, u:f32, v:f32) -> Self {
        let mut menu = Self {
            effect,
            rects: Vec::new(),
            u,
            v,
            size: 0.025};

        menu.add_rect();
        menu
    }

    pub fn add_rect(&mut self) {
        let x1=HelpItem::write_string(self.u, self.v, self.size, &self.effect, &mut self.rects);
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }

    ///Add RectVertexAttr from a string to a vector of RectVertexAttr and return the u position of the last character 
    pub fn write_string(u: f32, v: f32, w: f32, st: &String, rects: &mut Vec<RectVertexAttr>) -> f32 {
        // This function could probably be moved somewhere else
        for (i,c) in st.chars().enumerate() {
            if c== ' ' {continue}
            rects.push(RectVertexAttr::new_with_char(u + i as f32 * w * 3., v, w, GLChar::from_char(c)));
        }
        u + st.len() as f32 * w * 3.
    }

}

/// Implement the help menu
pub struct HelpMenu {
    items: Vec<HelpItem>,
    rects:Vec<RectVertexAttr>,
    y: f32
}

impl HelpMenu {
    pub fn new() -> Self {
        let mut helpmenu = HelpMenu {
            items:Vec::new(),
            rects: Vec::new(),
            y: 0.6
        };

        helpmenu.add_line(String::from("move forward"),String::from("z"));
        helpmenu.add_line(String::from("move backward"),String::from("s"));
        helpmenu.add_line(String::from("move left"),String::from("q"));
        helpmenu.add_line(String::from("move right"),String::from("d"));
        helpmenu.add_line(String::from("jump"),String::from("space"));
        helpmenu.add_line(String::from("help menu"),String::from("f12"));
        helpmenu.update();

        helpmenu
    }

    /// Add two text on the same line
    pub fn add_line(&mut self, string: String, string2: String) {
        self.add_item(string, -0.9);
        self.add_item_end_line(string2, 0.5);
    }

    pub fn add_item(&mut self, effect:String, u:f32) {
        self.items.push(HelpItem::new(effect,u,self.y));
    }

    pub fn add_item_end_line(&mut self, effect:String, u:f32) {
        self.items.push(HelpItem::new(effect,u,self.y));
        self.y-=0.1;
    }

    /// Recreate vectors of rectangle from the list of items
    pub fn update(&mut self) {
        self.rects=Vec::new();
        self.rects.push(RectVertexAttr::new_from_corner(-0.95, 0.70, 1.8, -0.7, LightCoral));
        for item in &self.items {
            self.rects.append(&mut item.rects().clone()); 
        }
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }
}