use crate::graphics::color::Color::{LighterGray, LightGray, Red};
use crate::graphics::inventory_event::InventoryEvent;
use crate::graphics::inventory_space;
use crate::graphics::inventory_space::{InventoryPosition, InventoryRect};
use crate::graphics::rectangle::RectInstance;
use crate::player_items::PlayerItems;
use crate::graphics::update_status::UpdateStatus;
use crate::graphics::inventory_slot::InventorySlot;
use winit::event::ElementState;
use crate::block_kind::Block;
use crate::crafting::{CraftingGrid, CraftingManager};

const INVENTORY_NROWS: usize = 4; // the 0th is the item bar
const INVENTORY_NCOLS: usize = 8;

pub struct InventoryMenu {
    rects: Vec<RectInstance>,
    aspect_ratio: f32,
    
    items: PlayerItems,
    cursor_pos: InventoryPosition,
    /// u, v, w, h of the UI, in NDC coordinates
    ui_rect: (f32, f32, f32, f32),

    inventory_slots: [[InventorySlot; INVENTORY_NCOLS]; INVENTORY_NROWS],
    crafting_slots: [[InventorySlot; 3]; 3],
    crafting_output_slot: InventorySlot,

    carried_item: Option<Block>,
    crafting_items: CraftingGrid,
    crafting_output_items: Option<Block>,
}

impl InventoryMenu {
    pub fn new(aspect_ratio: f32, items: PlayerItems) -> Self {
        let slot = InventorySlot::new(InventoryPosition::zero(), 1.);
        
        let mut inventory = Self {
            rects: Vec::new(),
            aspect_ratio,
            items,
            cursor_pos: InventoryPosition::zero(),
            ui_rect: (0., 0., 0., 0.),
            inventory_slots: [[slot; INVENTORY_NCOLS]; INVENTORY_NROWS],
            crafting_slots: [[slot; 3]; 3],
            crafting_output_slot: slot,

            carried_item: None,
            crafting_items: [[None; 3]; 3],
            crafting_output_items: None,
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

    pub fn take_items(self) -> PlayerItems {
        self.items
    }

    /// Inventory can be closed safely if there is no item in the crafting grid
    pub fn can_be_closed_safely(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.crafting_items[row][col].is_some() {
                    return false
                }
            }
        }
        
        true
    }
    
    pub fn handle_event(&mut self, event: InventoryEvent) -> UpdateStatus {
        match event {
            InventoryEvent::CursorMoved(x, y) => {
                if let Some(pos) = self.from_ndc_to_ui_position([x, y]) {
                    self.cursor_pos = pos;
                    self.update();

                    UpdateStatus::Update
                }
                else { UpdateStatus::NoUpdate }
            },
            InventoryEvent::Button(state) => {
                if state == ElementState::Pressed {
                    self.handle_button_pressed()
                }
                else { UpdateStatus::Update }
            }
        }
    }

    fn handle_button_pressed(&mut self) -> UpdateStatus {
        let mut update_craft = false;
        
        if let Some(carried_item) = self.carried_item {
            // if we have a valid slot, put it in

            // a valid slot is an inventory or crafting slot (no crafting
            // output), either empty or with an item of the same kind

            for row in 0..INVENTORY_NROWS {
                for col in 0..INVENTORY_NCOLS {
                    if self.inventory_slots[row][col].is_in(&self.cursor_pos) {
                        if row == 0 {
                            // for item bar
                            if self.items.put_bar_item(col, carried_item) {
                                self.carried_item = None;
                            }
                        }
                        else {
                            // for inventory
                            if self.items.put_inventory_item((row - 1) * INVENTORY_NCOLS + col,
                                                             carried_item) {
                                self.carried_item = None;
                            }
                        }
                    }
                }
            }

            // also check for crafting slots
            for row in 0..3 {
                for col in 0..3 {
                    if self.crafting_slots[row][col].is_in(&self.cursor_pos) {
                        if let None = self.crafting_items[row][col] {
                            std::mem::swap(&mut self.crafting_items[row][col],
                                           &mut self.carried_item);
                            update_craft = true;
                        }
                    }
                }
            }
        } else {
            // if we are in a non empty slot, take it

            for row in 0..INVENTORY_NROWS {
                for col in 0..INVENTORY_NCOLS {
                    if self.inventory_slots[row][col].is_in(&self.cursor_pos) {
                        // grab it
                        if row == 0 {
                            // for item bar
                            if let Some(block) = self.items.take_bar_item(col) {
                                self.carried_item = Some(block);
                            }
                        }
                        else {
                            // for inventory
                            if let Some(block) = self.items.take_inventory_item((row - 1) * INVENTORY_NCOLS + col) {
                                self.carried_item = Some(block);
                            }
                        }
                    }
                }
            }

            // also check for crafting slots
            for row in 0..3 {
                for col in 0..3 {
                    if self.crafting_slots[row][col].is_in(&self.cursor_pos) {
                        if let Some(_) = self.crafting_items[row][col] {
                            std::mem::swap(&mut self.crafting_items[row][col],
                                           &mut self.carried_item);
                            update_craft = true;
                        }
                    }
                }
            }

            // and for the crafting output
            if self.crafting_output_slot.is_in(&self.cursor_pos) {
                if let Some(_) = self.crafting_output_items {
                    std::mem::swap(&mut self.crafting_output_items,
                                   &mut self.carried_item);
                    // also remove the crafting components !
                    for row in 0..3 {
                        for col in 0..3 {
                            let _ = &mut self.crafting_items[row][col].take();
                        }
                    }
                    update_craft = true;
                }
            }
        }

        if update_craft { self.update_craft(); }

        self.update();
        UpdateStatus::Update
    }

    fn update(&mut self) {
        self.rects = Vec::new();

        // background of the inventory
        self.ui_rect = inventory_space::ui_boundaries(self.aspect_ratio);
        {
            let (u, v, w, h) = self.ui_rect;
            self.rects.push(
                RectInstance::new_from_corner(u, v, w, h, LightGray));
        }

        // inventory slots
        {
            let margin = 0.02;
            let item_size = (1. - margin * (INVENTORY_NCOLS as f32 + 1.)) / INVENTORY_NCOLS as f32;

            for row in 0..INVENTORY_NROWS {
                for col in 0..INVENTORY_NCOLS {
                    let item = if row == 0 {
                        // from item bar
                        self.items.get_bar_item(col)
                        
                    } else {
                        // inventory itself
                        self.items.get_inventory_item((row - 1) * 8 + col)
                    };
                    
                    let slot = InventorySlot::new(InventoryPosition::new(margin + col as f32 * (item_size + margin),
                                                                         margin + row as f32 * (item_size + margin)),
                                                  item_size);
                    
                    self.inventory_slots[row][col] = slot;
                    self.rects.append(&mut slot.rects(&self.ui_rect, item, slot.is_in(&self.cursor_pos)));
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

                    let item = self.crafting_items[row][col].map(|block| { (block, 1) });
                    self.rects.append(&mut slot.rects(&self.ui_rect, item, slot.is_in(&self.cursor_pos)));
                }
            }

            // crafting result
            {
                let slot = InventorySlot::new(InventoryPosition::new(craftx + 3.5 * (item_size + margin),
                                                                     crafty + 1 as f32 * (item_size + margin)),
                                              item_size);
                self.crafting_output_slot = slot;

                let item = self.crafting_output_items.map(|block| { (block, 1) });
                self.rects.append(&mut slot.rects(&self.ui_rect, item, slot.is_in(&self.cursor_pos)));
            }

            // carried item
            if let Some(block) = self.carried_item {
                let (x, y, w, h) = inventory_space::from_ui_to_ndc_rect(&self.ui_rect,
                                                                        &InventoryRect::new(self.cursor_pos.x,
                                                                                           self.cursor_pos.y,
                                                                                           item_size,
                                                                                           item_size));
                let mut rect = RectInstance::new_from_corner(x, y, w, h, Red);
                rect.set_block_id(block as u8 as i8);
                self.rects.push(rect);
            }
            
        }
    }

    /// Update the item in the output slot, depending on the recipe in the
    /// crafting slot. This only updates the logic, need to call update
    /// afterwards for the graphics
    fn update_craft(&mut self) {
        self.crafting_output_items = CraftingManager::recipe(&self.crafting_items)
    }

    /// Same as the function in `inventory_space.rs`, but using the UI rect from
    /// self
    fn from_ndc_to_ui_position(&self, vec: [f32; 2]) -> Option<InventoryPosition> {
        inventory_space::from_ndc_to_ui_position(&self.ui_rect, vec)
    }
    

}
