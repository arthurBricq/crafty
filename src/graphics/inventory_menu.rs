use crate::graphics::color::Color::LightGray;
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
        
        Self {
            rects: rects
        }
    }

    pub fn rects(&self) -> &Vec<RectInstance> {
        &self.rects
    }
}
