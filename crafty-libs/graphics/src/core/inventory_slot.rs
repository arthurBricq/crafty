use crate::core::inventory_space;
use crate::core::inventory_space::{InventoryPosition, InventoryRect};
use crate::core::string_rect::StringRect;
use crate::renderer::RectRenderData;
use model::game::player_items::ItemStack;
use primitives::color::Color::{EvenLighterGray, LighterGray};

#[derive(Debug, Clone, Copy)]
pub struct InventorySlot {
    pub position: InventoryPosition,
    pub size: f32,
}

impl InventorySlot {
    pub fn new(position: InventoryPosition, size: f32) -> Self {
        Self { position, size }
    }

    pub fn is_in(&self, position: &InventoryPosition) -> bool {
        self.position.x <= position.x
            && position.x <= self.position.x + self.size
            && self.position.y <= position.y
            && position.y <= self.position.y + self.size
    }

    pub fn rects(
        &self,
        ui_rect: &(f32, f32, f32, f32),
        item: Option<ItemStack>,
        hover: bool,
    ) -> Vec<RectRenderData> {
        let (x, y, w, h) = inventory_space::from_ui_to_ndc_rect(
            ui_rect,
            &InventoryRect::new(self.position.x, self.position.y, self.size, self.size),
        );

        let mut rects = Vec::new();
        // from_ui_to_ndc_rect returns corner coordinates (x, y) and full dimensions (w, h)
        let slot = RectRenderData::new_from_corner(x, y, w, h, if hover { EvenLighterGray } else { LighterGray });
        rects.push(slot);

        // draw the item as well
        if let Some((block, count)) = item {
            let mut item_rect = RectRenderData::new_from_corner(x, y, w, h, LighterGray);
            item_rect.block_id = Some(block as u8 as i8);
            rects.push(item_rect);

            // and the count
            let text = format!("{count}");
            let quantity = StringRect::new(&text, x, y, 0.03);
            rects.append(&mut quantity.rects().clone());
        }

        rects
    }
}
