/// A position in inventory space, i.e. from 0 to 1, origin on the bottom left
/// corner, with 0, 1 being the sides of the UI
#[derive(Debug, Clone, Copy)]
pub struct InventoryPosition {
    pub x: f32,
    pub y: f32
}

impl InventoryPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0., y: 0.}
    }
}

/// A size in inventory space, i.e. from 0 to 1, origin on the bottom left
/// corner, with 0, 1 being the sides of the UI. Different from
/// InventoryPosition because the change of coordinates from NDC is different
#[derive(Debug)]
pub struct InventorySize {
    pub w: f32,
    pub h: f32
}

impl InventorySize {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}

/// A combination of both InventoryPosition and InventorySize
#[derive(Debug)]
pub struct InventoryRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl InventoryRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

/// Returns the `(x, y, w, h)` boundaries of the ui for a given aspect
/// ratio. This is because we want the menus to always be at fixed aspect ratio,
/// while the window ratio is free to change. 
pub fn ui_boundaries(aspect_ratio: f32) -> (f32, f32, f32, f32) {
    let margin_h: f32 = 0.1; // this will stay constant; compute the other margins from that
    let target_ratio: f32 = 1.; // the UI actually looks good as a square

    let margin_w: f32 = 1. - (1. - margin_h) * target_ratio / aspect_ratio;
    
    (-1. + margin_w, -1. + margin_h, 2. - 2. * margin_w, 2. - 2. * margin_h)
}

/// Try to convert a NDC position to inventory space. This may fail since the UI
/// does not cover the full window.
pub fn from_ndc_to_ui_position(ui_rect: &(f32, f32, f32, f32), vec: [f32; 2])
                           -> Option<InventoryPosition> {
    let (xui, yui, wui, hui) = ui_rect;
    let [x, y] = vec;
    let u = (x - xui) / wui;
    let v = (y - yui) / hui;

    if 0. <= u && u <= 1. && 0. <= v && v <= 1. {
        Some(InventoryPosition::new(u, v))
    } else { None }
}


/// Convert a InventoryRect to NDC.
pub fn from_ui_to_ndc_rect(ui_rect: &(f32, f32, f32, f32), rect: &InventoryRect) -> (f32, f32, f32, f32) {
    let (xui, yui, wui, hui) = ui_rect;
    let InventoryRect {x, y, w, h} = rect;
    
    (
        xui + wui * x,
        yui + hui * y,
        wui * w,
        hui * h,
    )
}
