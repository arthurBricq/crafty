use crate::graphics::color::Color::LightGray;
use crate::graphics::color::Color::LightCoral;
use crate::graphics::font::GLChar;
use crate::graphics::rectangle::RectVertexAttr;


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

/// A tile is a rectangle drawn on the screen, such as a menu.
pub struct HUDManager {
    /// List of the tiles to be presented on the screen
    rects: Vec<RectVertexAttr>,
    base: Vec<RectVertexAttr>,
    helpmenu: HelpMenu,
    showhelp: bool
}

impl HUDManager {
    pub fn new() -> Self {
        let mut hud= Self { 
            rects: Vec::new(),
            base: Vec::new(),
            helpmenu: HelpMenu::new(),
            showhelp: false
        };
        hud.add_cross();
        //hud.add_crafty_label();
        hud.update();



        hud
    }

    /// Adds a cross in the center of the screen
    pub fn add_cross(&mut self) {
        let w = 0.05;
        let s = 0.01;
        self.base.push(RectVertexAttr::new(0., 0., w / 1.5 , s, LightGray));
        self.base.push(RectVertexAttr::new(0., 0., s / 2.5, w, LightGray));
    }
    
    pub fn add_crafty_label(&mut self) {
        let h = 0.4;
        let s = 0.05;
        let x0 = -0.3;
        self.base.push(RectVertexAttr::new_with_char(x0, h, s, GLChar::C));
        self.base.push(RectVertexAttr::new_with_char(x0 + 1. * s * 3., h, s, GLChar::R));
        self.base.push(RectVertexAttr::new_with_char(x0 + 2. * s * 3., h, s, GLChar::A));
        self.base.push(RectVertexAttr::new_with_char(x0 + 3. * s * 3., h, s, GLChar::F));
        self.base.push(RectVertexAttr::new_with_char(x0 + 4. * s * 3., h, s, GLChar::T));
        self.base.push(RectVertexAttr::new_with_char(x0 + 5. * s * 3., h, s, GLChar::Y));
    }

    ///Switch the bool for showing the help menu
    pub fn toggle_help_menu(&mut self) {
        if self.showhelp {self.showhelp = false}
        else {self.showhelp = true}
        self.update();
    }

    ///Update the vector of RectVertexAttr to be shown
    pub fn update(&mut self) {
        self.rects=self.base.clone();
        if self.showhelp {
            self.rects.append(&mut self.helpmenu.rects().clone());
        }
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }
}