pub mod cube;
pub mod cube_instance;
pub mod entity;
pub mod font;
pub mod rectangle;

pub use cube_instance::CubeInstance;
pub use entity::EntityCube;
pub use rectangle::RectInstance;

use primitives::render_data::{CubeRenderData, EntityRenderData, RectRenderData};

impl From<CubeRenderData> for CubeInstance {
    fn from(data: CubeRenderData) -> Self {
        if data.is_selected {
            CubeInstance::new_selected(data.position, data.block_id)
        } else {
            CubeInstance::new(data.position, data.block_id)
        }
    }
}

impl From<EntityRenderData> for EntityCube {
    fn from(data: EntityRenderData) -> Self {
        // Body part 0 (head) needs full rotation, others use yaw only
        match data.body_part_id {
            0 => EntityCube::new(&data.position, data.body_part_id, data.monster_type, data.scale),
            _ => EntityCube::new_only_yaw(&data.position, data.body_part_id, data.monster_type, data.scale),
        }
    }
}

impl From<RectRenderData> for RectInstance {
    fn from(data: RectRenderData) -> Self {
        if data.is_font {
            RectInstance::new_with_font_coords(
                data.u,
                data.v,
                data.w,
                data.font_coords.unwrap_or([0., 0.]),
            )
        } else {
            let mut rect = RectInstance::new(data.u, data.v, data.w, data.h, data.color);
            if let Some(block_id) = data.block_id {
                rect.set_block_id(block_id);
            }
            rect
        }
    }
}
