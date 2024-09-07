use crate::graphics::color::Color::{LightCoral, LightGray, Red};
use crate::graphics::rectangle::RectInstance;
use crate::graphics::string_rect::StringRect;
use crate::player_items::Items;

pub struct ItemBar {
    items: Items,
    selected_item: usize, 
    rects: Vec<RectInstance>,
    aspect_ratio: f32
}

impl ItemBar {
    pub fn new() -> Self {
        let menu = Self {
            items: Vec::new(),
            selected_item: 0,
            aspect_ratio: 2.0,
            rects: Vec::new()
        };
        menu
    }

    /// Sets the items currently showed on the bar
    pub fn set_items(&mut self, items: Items) {
        self.items = items;

        const W: f32 = 1.8;
        const H: f32 = 0.3;
        const PADDING: f32 = 0.01;
        const BOTTOM: f32 = 0.1;

        // Add the background tiles
        let mut rects = Vec::new();
        let background = RectInstance::new_from_corner(-W / 2., BOTTOM - 1., W, H + 2. * PADDING, LightGray);
        rects.push(background);

        // Add the items
        let x0 = -W / 2. + PADDING;
        const ITEM_SIDE: f32 = H * 0.9;
        for (i, &(kind, quantity)) in self.items.iter().enumerate() {
            let color = if (i == self.selected_item) { Red } else { LightCoral };
            
            // We want to do a square
            let a = ITEM_SIDE / self.aspect_ratio;
            let b = ITEM_SIDE;
            let mut cube = RectInstance::new_from_corner(x0 + (i as f32) * (ITEM_SIDE), BOTTOM - 1. + 2. * PADDING, a, b, color);
            cube.set_block_id(kind as u8 as i8);
            rects.push(cube);
            
            // And we want to print the number of remaining items
            let text = format!("{quantity}");
            let quantity = StringRect::new(&text, x0 + ITEM_SIDE / 5. + (i as f32) * (ITEM_SIDE) , BOTTOM - 1. + 0. * PADDING, 0.03);
            rects.append(&mut quantity.rects().clone())
        }

        self.rects = rects;
    }

    pub fn rects(&self) -> Vec<RectInstance> {
        self.rects.clone()
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
        self.set_items(self.items.clone());
    }

}