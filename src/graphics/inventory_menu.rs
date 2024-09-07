use crate::graphics::color::Color::{LighterGray, LightGray};
use crate::graphics::rectangle::RectInstance;

pub struct InventoryMenu {
    rects: Vec<RectInstance>,
}

impl InventoryMenu {
    pub fn new() -> Self {
        let mut rects = Vec::new();

        // background of the inventory
        const MARGIN_W: f32 = 0.1;
        const MARGIN_H: f32 = 0.1;
        rects.push(
            RectInstance::new_from_corner(-1. + MARGIN_W,
                                          -1. + MARGIN_H,
                                          2. - 2. * MARGIN_W,
                                          2. - 2. * MARGIN_H,
                                          LightGray));

        // we want a 4 rows * 8 columns item grid for the inventory, and item
        // bar
        let inner_margin = 0.05;
        let startx = -1. + MARGIN_W + inner_margin;
        // because margin taken into account by item slot
        let endx = 1. - MARGIN_W;

        let starty = -1. + MARGIN_H + inner_margin;
        let endy = 1. - MARGIN_H - 1.; // same

        let nrows = 4;
        let ncolumns = 8;

        let slotw_with_margin = (endx - startx) / ncolumns as f32;
        let sloth_with_margin = (endy - starty) / nrows as f32;

        for row in 0..nrows {
            for column in 0..ncolumns {
                rects.push(
                    RectInstance::new_from_corner(startx + column as f32 * slotw_with_margin,
                                                  starty + row as f32 * sloth_with_margin,
                                                  slotw_with_margin - inner_margin,
                                                  sloth_with_margin - inner_margin,
                                                  LighterGray));
            }
        }

        // crafting menu
        rects.push(
            RectInstance::new_from_corner(-1. + MARGIN_W + 0.9,
                                          -1. + MARGIN_H + 0.9,
                                          0.7, 0.7,
                                          LighterGray));

        Self {
            rects: rects
        }
    }

    pub fn rects(&self) -> &Vec<RectInstance> {
        &self.rects
    }
}
