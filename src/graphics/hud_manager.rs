use crate::graphics::color::Color::LightGray;
use crate::graphics::color::Color::LightCoral;
use crate::graphics::font::GLChar;
use crate::graphics::rectangle::RectVertexAttr;
use crate::graphics::menu_help::HelpMenu;

use super::menu_help::HelpMenuData;
use super::menu_help::HelpMenuItem;


/// A tile is a rectangle drawn on the screen, such as a menu.
pub struct HUDManager {
    /// List of the tiles to be presented on the screen
    rects: Vec<RectVertexAttr>,
    base: Vec<RectVertexAttr>,
    help_menu: HelpMenu,
    help_menu_data: HelpMenuData,
    show_help: bool
}

impl HUDManager {
    pub fn new() -> Self {

        // To be read from a file
        let mut items= Vec::new();
        let mut help_menu_data= HelpMenuData::new(items);
        help_menu_data.add_item(String::from("move forward"),String::from("z"));
        help_menu_data.add_item(String::from("move backward"),String::from("s"));
        help_menu_data.add_item(String::from("move left"),String::from("q"));
        help_menu_data.add_item(String::from("move right"),String::from("d"));
        help_menu_data.add_item(String::from("jump"),String::from("space"));
        help_menu_data.add_item(String::from("help menu"),String::from("f12"));

        let mut hud= Self { 
            rects: Vec::new(),
            base: Vec::new(),
            help_menu: HelpMenu::new(&help_menu_data),
            help_menu_data: help_menu_data,
            show_help: false
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

    /// Add/Remove the help menu
    pub fn toggle_help_menu(&mut self) {
        if self.show_help {self.show_help = false}
        else {self.show_help = true}
        self.update();
    }

    /// Update the vector of RectVertexAttr to be shown
    fn update(&mut self) {
        self.rects=self.base.clone();
        if self.show_help {
            self.rects.append(&mut self.help_menu.rects().clone());
        }
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }
}