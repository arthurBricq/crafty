pub mod renderer;

pub mod core {
    pub mod entity;
    pub mod health_bar;
    pub mod hud_renderer;
    pub mod inventory_event;
    pub mod inventory_menu;
    pub mod inventory_slot;
    pub mod inventory_space;
    pub mod items_bar;
    pub mod menu_debug;
    pub mod menu_help;
    pub mod string_rect;
    pub mod update_status;
}

pub mod player {
    pub mod fps;
    pub mod world_renderer;
}
