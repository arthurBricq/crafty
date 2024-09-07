use crate::graphics::color::Color::{LightGray, LightYellow, Red};
use crate::graphics::rectangle::RectInstance;
use crate::graphics::string_rect::StringRect;
use crate::player_items::Items;

pub struct ItemBar {
    items: Items,
    selected_item: usize,
    rects: Vec<RectInstance>,
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
    pub fn set_items(&mut self, items: Items, selected: usize) {
        self.items = items;
        self.selected_item = selected;

        const W: f32 = 1.8;
        const H: f32 = 0.3;
        const PADDING: f32 = 0.01;
        const BOTTOM: f32 = 0.1;

        // Add the background tiles
        let mut rects = Vec::new();
        let background = RectInstance::new_from_corner(-W / 2., BOTTOM - 1., W, H + 2. * PADDING, LightGray);
        rects.push(background);

        // Add the items
        let mut x0 = -W / 2. + PADDING;

        const ITEM_SIDE: f32 = H * 0.9;
        for (i, &(kind, quantity)) in self.items.iter().enumerate() {
            if i == self.selected_item {
                // Add an indication that this is the selected item
                const DX: f32 = 0.015;
                let cube = RectInstance::square_from_corner(
                    x0 - DX / self.aspect_ratio,
                    BOTTOM - 1. + 2. * PADDING - DX,
                    ITEM_SIDE + 2. * DX, self.aspect_ratio, LightYellow);
                rects.push(cube);
            }

            let mut cube = RectInstance::square_from_corner(
                x0,
                BOTTOM - 1. + 2. * PADDING,
                ITEM_SIDE, self.aspect_ratio, Red);
            cube.set_block_id(kind as u8 as i8);
            rects.push(cube);

            // And we want to print the number of remaining items
            let text = format!("{quantity}");
            let quantity = StringRect::new(&text, x0 + ITEM_SIDE / 5., BOTTOM - 1. + 0. * PADDING, 0.03);
            rects.append(&mut quantity.rects().clone());

            x0 += ITEM_SIDE;
        }

        self.rects = rects;
    }

    pub fn rects(&self) -> Vec<RectInstance> {
        self.rects.clone()
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
        self.set_items(self.items.clone(), self.selected_item);
    }
}