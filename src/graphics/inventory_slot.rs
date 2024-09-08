use crate::graphics::inventory_menu::InventoryMenu;
use crate::graphics::inventory_space;
use crate::graphics::inventory_space::{InventoryPosition, InventoryRect};
use crate::graphics::rectangle::RectInstance;
use crate::graphics::color::Color::{LighterGray, EvenLighterGray};
use crate::player_items::ItemStack;
use crate::graphics::string_rect::StringRect;

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
    
    pub fn rects(&self, ui_rect: &(f32, f32, f32, f32), item: Option<ItemStack>, hover: bool) -> Vec<RectInstance> {
        let (x, y, w, h) =
            inventory_space::from_ui_to_ndc_rect(ui_rect, &InventoryRect::new(self.position.x,
                                                                              self.position.y,
                                                                              self.size,
                                                                              self.size));
        
        let mut rects = Vec::new();
        rects.push(RectInstance::new_from_corner(x, y, w, h, if hover { EvenLighterGray } else { LighterGray }));

        // draw the item as well
        if let Some((block, count)) = item {
            let mut rect = RectInstance::new_from_corner(x, y, w, h, LighterGray);
            rect.set_block_id(block as u8 as i8);
            rects.push(rect);

            // and the count
            let text = format!("{count}");
            let quantity = StringRect::new(&text, x, y, 0.03);
            rects.append(&mut quantity.rects().clone());
        }

        rects
    }
}
