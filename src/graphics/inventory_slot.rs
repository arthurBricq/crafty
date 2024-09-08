use crate::graphics::inventory_menu::InventoryMenu;
use crate::graphics::inventory_menu::InventoryPosition;
use crate::graphics::rectangle::RectInstance;
use crate::graphics::color::Color::{LighterGray, EvenLighterGray};

#[derive(Debug, Clone, Copy)]
pub struct InventorySlot {
    pub position: InventoryPosition,
    pub size: f32
}

impl InventorySlot {
    pub fn new(position: InventoryPosition, size: f32) -> Self {
        Self { position, size }
    }

    pub fn is_in(&self, position: &InventoryPosition) -> bool {
        self.position.x <= position.x &&
            position.x <= self.position.x + self.size &&
            self.position.y <= position.y &&
            position.y <= self.position.y + self.size
    }
    
    pub fn rect(&self, ui_rect: &(f32, f32, f32, f32), hover: bool) -> RectInstance {
        let (x, y, w, h) =
            InventoryMenu::from_ui_to_ndc_rect(ui_rect, &(self.position.x,
                                                          self.position.y,
                                                          self.size,
                                                          self.size));

        RectInstance::new_from_corner(x, y, w, h, if hover { EvenLighterGray } else { LighterGray })
    }
}
