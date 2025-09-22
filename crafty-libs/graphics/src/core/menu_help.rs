use primitives::opengl::rectangle::RectInstance;
use crate::core::string_rect::StringRect;

/// Data in the help menu
pub const HELP_MENU_DATA: [HelpMenuItem; 9] = [
    //HelpMenuItem{command: &str "move forward",key: String::from("z")},
    HelpMenuItem::new("move forward", "z"),
    HelpMenuItem::new("move backward", "s"),
    HelpMenuItem::new("move left", "q"),
    HelpMenuItem::new("move right", "d"),
    HelpMenuItem::new("jump", "space"),
    HelpMenuItem::new("debug menu", "f3"),
    HelpMenuItem::new("help menu", "f12"),
    HelpMenuItem::new("save map", "f10"),
    HelpMenuItem::new("fullscreen", "f11"),
];

#[derive(Clone, Copy)]
pub struct HelpMenuItem {
    command: &'static str,
    key: &'static str,
}

impl HelpMenuItem {
    pub const fn new(command: &'static str, key: &'static str) -> Self {
        Self { command, key }
    }
    pub fn command(&self) -> &str {
        &self.command
    }
    pub fn key(&self) -> &str {
        &self.key
    }
}

pub struct HelpMenuData {
    items: Vec<HelpMenuItem>,
}

impl HelpMenuData {
    pub fn new(items: Vec<HelpMenuItem>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &Vec<HelpMenuItem> {
        &self.items
    }
}

/// Implement the help menu
pub struct HelpMenu {
    rects: Vec<RectInstance>,
}

impl HelpMenu {
    pub fn new(help_menu_data: &HelpMenuData) -> Self {
        let mut rects = Vec::new();
        let mut y = 0.6;
        let u = -0.9;
        let size = 0.025;
        for item in help_menu_data.items() {
            StringRect::write_string(u, y, size, &item.command().to_string(), &mut rects);
            StringRect::write_string(u + 0.98, y, size, &item.key().to_string(), &mut rects);
            y -= 4. * size;
        }
        Self { rects }
    }
    pub fn rects(&self) -> &Vec<RectInstance> {
        &self.rects
    }
}
