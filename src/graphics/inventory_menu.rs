use crate::graphics::color::Color::{LighterGray, LightGray, Red};
use crate::graphics::inventory_event::InventoryEvent;
use crate::graphics::rectangle::RectInstance;
use crate::player_items::PlayerItems;
use crate::graphics::update_status::UpdateStatus;
use crate::graphics::inventory_slot::InventorySlot;

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

pub struct InventoryMenu {
    rects: Vec<RectInstance>,
    aspect_ratio: f32,
    
    player_items: PlayerItems,
    cursor_pos: InventoryPosition,
    /// u, v, w, h of the UI, in NDC coordinates
    ui_rect: (f32, f32, f32, f32),

    inventory_slots: [[InventorySlot; 8]; 4],
    crafting_slots: [[InventorySlot; 3]; 3],
    crafting_output_slot: InventorySlot,
}

impl InventoryMenu {
    pub fn new(aspect_ratio: f32, player_items: PlayerItems) -> Self {
        let slot = InventorySlot::new(InventoryPosition::zero(), 1.);
        
        let mut inventory = Self {
            rects: Vec::new(),
            aspect_ratio,
            player_items,
            cursor_pos: InventoryPosition::zero(),
            ui_rect: (0., 0., 0., 0.),
            inventory_slots: [[slot; 8]; 4],
            crafting_slots: [[slot; 3]; 3],
            crafting_output_slot: slot,
        };
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

    pub fn take_player_items(self) -> PlayerItems {
        self.player_items
    }

    pub fn handle_event(&mut self, event: InventoryEvent) -> UpdateStatus {
        match event {
            InventoryEvent::CursorMoved(x, y) => {
                if let Some(pos) = self.from_ndc_to_inventory_position([x, y]) {
                    self.cursor_pos = pos;
                    self.update();

                    UpdateStatus::Update
                }
                else { UpdateStatus::NoUpdate }
            },
            InventoryEvent::Button(state) => {
                dbg!(state);

                UpdateStatus::NoUpdate
            }
        }
    }

    fn update(&mut self) {
        self.rects = Vec::new();

        // background of the inventory
        self.ui_rect = Self::ui_boundaries(self.aspect_ratio);
        {
            let (u, v, w, h) = self.ui_rect;
            self.rects.push(
                RectInstance::new_from_corner(u, v, w, h, LightGray));
        }

        // inventory slots
        {
            let ncols = 8;
            let nrows = 4;
            let margin = 0.02;
            let item_size = (1. - margin * (ncols as f32 + 1.)) / ncols as f32;

            for row in 0..nrows {
                for col in 0..ncols {
                    let slot = InventorySlot::new(InventoryPosition::new(margin + col as f32 * (item_size + margin),
                                                                         margin + row as f32 * (item_size + margin)),
                                                  item_size);
                    
                    self.inventory_slots[row][col] = slot;
                    self.rects.push(slot.rect(&self.ui_rect, slot.is_in(&self.cursor_pos)));
                }
            }

            // draw crafting grid
            let craftx = 0.4;
            let crafty = 0.6;

            for row in 0..3 {
                for col in 0..3 {
                    let slot = InventorySlot::new(InventoryPosition::new(craftx + col as f32 * (item_size + margin),
                                                                         crafty + row as f32 * (item_size + margin)),
                                                  item_size);

                    self.crafting_slots[row][col] = slot;
                    self.rects.push(slot.rect(&self.ui_rect, slot.is_in(&self.cursor_pos)));
                }
            }

            // crafting result
            {
                let slot = InventorySlot::new(InventoryPosition::new(craftx + 3.5 * (item_size + margin),
                                                                     crafty + 1 as f32 * (item_size + margin)),
                                              item_size);
                self.crafting_output_slot = slot;
                self.rects.push(slot.rect(&self.ui_rect, slot.is_in(&self.cursor_pos)));
            }

            // tmp
            {
                let (x, y, w, h) = Self::from_ui_to_ndc_rect(&self.ui_rect, &(self.cursor_pos.x,
                                                                              self.cursor_pos.y,
                                                                              item_size,
                                                                              item_size));
                self.rects.push(
                    RectInstance::new_from_corner(x, y, w, h, Red));
            }
            
        }
    }

    fn from_ndc_to_inventory_position(&self, vec: [f32; 2]) -> Option<InventoryPosition> {
        let (xui, yui, wui, hui) = self.ui_rect;
        let [x, y] = vec;
        let u = (x - xui) / wui;
        let v = (y - yui) / hui;

        if 0. <= u && u <= 1. && 0. <= v && v <= 1. {
            Some(InventoryPosition::new(u, v))
        } else { None }
    }
    
    /// Returns the `(x, y, w, h)` boundaries of the ui for a given aspect ratio
    fn ui_boundaries(aspect_ratio: f32) -> (f32, f32, f32, f32) {
        let margin_h: f32 = 0.1; // this will be fixed; compute the other margins from that
        let target_ratio: f32 = 1.; // for now

        let margin_w: f32 = 1. - (1. - margin_h) * target_ratio / aspect_ratio;
        
        (-1. + margin_w, -1. + margin_h, 2. - 2. * margin_w, 2. - 2. * margin_h)
    }

    pub fn from_ui_to_ndc_rect(ui_rect: &(f32, f32, f32, f32), rect: &(f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
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
