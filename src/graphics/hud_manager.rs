use crate::graphics::color::Color::LightGray;
use crate::graphics::color::Color::LightCoral;
use crate::graphics::font::GLChar;
use crate::graphics::rectangle::RectVertexAttr;

use super::menu_help;
use super::menu_help::HelpMenu;
use super::menu_help::HelpMenuData;
use super::menu_help::HelpMenuItem;

use super::menu_debug;
use super::menu_debug::DebugData;
use super::menu_debug::DebugMenu;
use super::menu_debug::DebugMenuData;

use crate::items_bar::ItemBar;
use crate::player_items::Items;

pub trait RectProvider {
    fn rects(&self) -> &Vec<RectVertexAttr>;
}


/// A tile is a rectangle drawn on the screen, such as a menu.
pub struct HUDManager {
    /// Ratio of the W over the H
    aspect_ratio: f32,
    /// List of the tiles to be presented on the screen
    rects: Vec<RectVertexAttr>,
    /// The rects that are always present on the screen
    base: Vec<RectVertexAttr>,

    help_menu_data: HelpMenuData,
    help_menu: HelpMenu,
    show_help: bool,

    debug_menu_data: DebugMenuData,
    debug_menu: DebugMenu,
    show_debug: bool,

    items_bar: ItemBar
}

impl HUDManager {
    pub fn new() -> Self {

        let mut help_menu_data= HelpMenuData::new(menu_help::HELP_MENU_DATA.to_vec());
        
        let mut debug_menu_data= DebugMenuData::new(menu_debug::DEBUG_MENU_DATA.to_vec());

        let mut hud= Self { 
            aspect_ratio: 1.0,
            rects: Vec::new(),
            base: Vec::new(),
            help_menu: HelpMenu::new(&help_menu_data),
            help_menu_data: help_menu_data,
            debug_menu: DebugMenu::new(&debug_menu_data),
            debug_menu_data,
            show_help: false,
            show_debug: false,
            items_bar: ItemBar::new(),
        };

        hud.add_cross();
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
        self.show_help= !self.show_help;
        self.update();
    }

    /// Add/Remove the debug menu
    pub fn toggle_debug_menu(&mut self) {
        self.show_debug= !self.show_debug;
        self.update();
    }

    /// Update the vector of RectVertexAttr to be shown
    fn update(&mut self) {
        // We first clone and append the Vec in each menu
        // and then do it again here, maybe we can only do it here ?
        // rects() would return a Vec of ref to append
        self.rects=self.base.clone();
        self.rects.append(&mut self.items_bar.rects());
        if self.show_help {
            self.rects.append(&mut self.help_menu.rects().clone());
        }
        if self.show_debug {
            self.rects.append(&mut self.debug_menu.rects().clone());
        }
    }

    pub fn set_debug(&mut self, debug_data: DebugData) {
        self.debug_menu.set_items(debug_data);
        self.update();
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }
    
    pub fn show_debug(&self) -> bool {
        self.show_debug
    }

    pub fn set_dimension(&mut self, dim: (u32, u32)) {
        println!("dimension={dim:?}");
        self.aspect_ratio = dim.0 as f32 / dim.1 as f32;
        self.items_bar.set_aspect_ratio(self.aspect_ratio);
        self.update();
    }

    pub fn set_player_items(&mut self, items: Items) {
        self.items_bar.set_items(items);
        self.update();
    }
}