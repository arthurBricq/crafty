use crate::graphics::color::Color::{LighterGray, LightGray};
use crate::graphics::rectangle::RectInstance;

pub struct InventoryMenu {
    rects: Vec<RectInstance>,
    aspect_ratio: f32,
}

impl InventoryMenu {
    pub fn new(aspect_ratio: f32) -> Self {
        let mut inventory = Self { rects: Vec::new(), aspect_ratio };
        inventory.update();

        inventory
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.update()
    }

    pub fn rects(&self) -> &Vec<RectInstance> {
        &self.rects
    }

    fn update(&mut self) {
        self.rects = Vec::new();

        // background of the inventory
        let ui_rect = Self::ui_boundaries(self.aspect_ratio);
        {
            let (u, v, w, h) = ui_rect;
            self.rects.push(
                RectInstance::new_from_corner(u, v, w, h, LightGray));
        }

        // draw item slots
        {
            let ncols = 8;
            let nrows = 4;
            let margin = 0.02;
            let item_size = (1. - margin * (ncols as f32 + 1.)) / ncols as f32;

            for row in 0..nrows {
                for col in 0..ncols {
                    let (x, y, w, h) = Self::from_ui_to_ndc_rect(&ui_rect, &(margin + col as f32 * (item_size + margin),
                                                                             margin + row as f32 * (item_size + margin),
                                                                             item_size,
                                                                             item_size));
                    self.rects.push(
                        RectInstance::new_from_corner(x, y, w, h, LighterGray));
                }
            }

            // draw crafting grid
            let craftx = 0.4;
            let crafty = 0.6;

            for row in 0..3 {
                for col in 0..3 {
                    let (x, y, w, h) = Self::from_ui_to_ndc_rect(&ui_rect, &(craftx + col as f32 * (item_size + margin),
                                                                             crafty + row as f32 * (item_size + margin),
                                                                             item_size,
                                                                             item_size));
                    self.rects.push(
                        RectInstance::new_from_corner(x, y, w, h, LighterGray));
                }
            }

            // crafting result
            {
                let (x, y, w, h) = Self::from_ui_to_ndc_rect(&ui_rect, &(craftx + 3.5 * (item_size + margin),
                                                                         crafty + 1 as f32 * (item_size + margin),
                                                                         item_size,
                                                                         item_size));
                self.rects.push(
                    RectInstance::new_from_corner(x, y, w, h, LighterGray));
            }
        }
    }

    /// Returns the `(x, y, w, h)` boundaries of the ui for a given aspect ratio
    fn ui_boundaries(aspect_ratio: f32) -> (f32, f32, f32, f32) {
        let margin_h: f32 = 0.1; // this will be fixed; compute the other margins from that
        let target_ratio: f32 = 1.; // for now

        let margin_w: f32 = 1. - (1. - margin_h) * target_ratio / aspect_ratio;
        dbg!(margin_h);
        
        (-1. + margin_w, -1. + margin_h, 2. - 2. * margin_w, 2. - 2. * margin_h)
    }

    fn from_ui_to_ndc_rect(ui_rect: &(f32, f32, f32, f32), rect: &(f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
        let (xui, yui, wui, hui) = ui_rect;
        let (x, y, w, h) = rect;
        
        (
            xui + wui * x,
            yui + hui * y,
            wui * w,
            hui * h,
        )
    }
}
