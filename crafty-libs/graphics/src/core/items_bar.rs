use crate::core::string_rect::StringRect;
use crate::renderer::RectRenderData;
use model::game::player_items::ItemStack;
use primitives::color::Color::{LightGray, LightYellow, Red};

pub struct ItemBar {
    items: Vec<ItemStack>,
    selected_item: usize,
    rects: Vec<RectRenderData>,
    aspect_ratio: f32,
}

impl ItemBar {
    pub fn new() -> Self {
        let menu = Self {
            items: Vec::new(),
            selected_item: 0,
            aspect_ratio: 2.0,
            rects: Vec::new(),
        };
        menu
    }

    /// Sets the items currently showed on the bar
    pub fn set_items(&mut self, items: Vec<ItemStack>, selected: usize) {
        self.items = items;
        self.selected_item = selected;

        const W: f32 = 1.8;
        const H: f32 = 0.3;
        const PADDING: f32 = 0.01;
        const BOTTOM: f32 = 0.1;

        // Add the background tiles
        let mut rects = Vec::new();
        // Background: corner coordinates
        let bg_u = -W / 2.;
        let bg_v = BOTTOM - 1.;
        let bg_w = W;
        let bg_h = H + 2. * PADDING;
        rects.push(RectRenderData::new_from_corner(bg_u, bg_v, bg_w, bg_h, LightGray));

        // Add the items
        let mut x0 = -W / 2. + PADDING;

        const ITEM_SIDE: f32 = H * 0.9;
        for (i, &(kind, quantity)) in self.items.iter().enumerate() {
            if i == self.selected_item {
                // Add an indication that this is the selected item
                const DX: f32 = 0.015;
                let sel_u = x0 - DX / self.aspect_ratio;
                let sel_v = BOTTOM - 1. + 2. * PADDING - DX;
                let sel_size = ITEM_SIDE + 2. * DX;
                rects.push(RectRenderData::square_from_corner(sel_u, sel_v, sel_size, self.aspect_ratio, LightYellow));
            }

            // Item slot: corner coordinates
            let item_u = x0;
            let item_v = BOTTOM - 1. + 2. * PADDING;
            let mut item_rect = RectRenderData::square_from_corner(item_u, item_v, ITEM_SIDE, self.aspect_ratio, Red);
            item_rect.block_id = Some(kind as u8 as i8);
            rects.push(item_rect);

            // And we want to print the number of remaining items
            let text = format!("{quantity}");
            let quantity =
                StringRect::new(&text, x0 + ITEM_SIDE / 5., BOTTOM - 1. + 0. * PADDING, 0.03);
            rects.append(&mut quantity.rects().clone());

            x0 += ITEM_SIDE;
        }

        self.rects = rects;
    }

    pub fn rects(&self) -> Vec<RectRenderData> {
        self.rects.clone()
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
        self.set_items(self.items.clone(), self.selected_item);
    }
}
