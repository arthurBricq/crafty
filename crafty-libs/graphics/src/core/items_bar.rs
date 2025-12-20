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
        let bg_u = -W / 2. + W / 2.;
        let bg_v = BOTTOM - 1. + (H + 2. * PADDING) / 2.;
        let bg_w = W / 2.;
        let bg_h = (H + 2. * PADDING) / 2.;
        rects.push(RectRenderData {
            u: bg_u,
            v: bg_v,
            w: bg_w,
            h: bg_h,
            color: LightGray,
            is_font: false,
            font_coords: None,
            block_id: None,
        });

        // Add the items
        let mut x0 = -W / 2. + PADDING;

        const ITEM_SIDE: f32 = H * 0.9;
        for (i, &(kind, quantity)) in self.items.iter().enumerate() {
            if i == self.selected_item {
                // Add an indication that this is the selected item
                const DX: f32 = 0.015;
                let sel_u = x0 - DX / self.aspect_ratio + (ITEM_SIDE + 2. * DX) / 2.;
                let sel_v = BOTTOM - 1. + 2. * PADDING - DX + (ITEM_SIDE + 2. * DX) / 2.;
                let sel_w = (ITEM_SIDE + 2. * DX) / 2.;
                let sel_h = (ITEM_SIDE + 2. * DX) / 2.;
                rects.push(RectRenderData {
                    u: sel_u,
                    v: sel_v,
                    w: sel_w,
                    h: sel_h,
                    color: LightYellow,
                    is_font: false,
                    font_coords: None,
                    block_id: None,
                });
            }

            let item_u = x0 + ITEM_SIDE / 2.;
            let item_v = BOTTOM - 1. + 2. * PADDING + ITEM_SIDE / 2.;
            let item_w = ITEM_SIDE / 2.;
            let item_h = ITEM_SIDE / 2.;
            rects.push(RectRenderData {
                u: item_u,
                v: item_v,
                w: item_w,
                h: item_h,
                color: Red,
                is_font: false,
                font_coords: None,
                block_id: Some(kind as u8 as i8),
            });

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
