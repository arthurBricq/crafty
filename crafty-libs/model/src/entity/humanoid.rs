use crate::collision::aabb::AABB;
use crate::game::player::{DIAMETER, FOREHEAD, PLAYER_HEIGHT};
use primitives::opengl::entity::EntityCube;
use primitives::position::Position;
use primitives::vector::Vector3;

// Define some constants to draw a player
// Height taken from human proportion in drawing
const PLAYER_HEAD_SIZE: f32 = 0.190 * PLAYER_HEIGHT;
const PLAYER_BODY_HEIGHT: f32 = 0.360 * PLAYER_HEIGHT;
const PLAYER_LEG_HEIGHT: f32 = 0.450 * PLAYER_HEIGHT;
const PLAYER_ARM_HEIGHT: f32 = 0.360 * PLAYER_HEIGHT;

const PLAYER_BODY_SHIFT: f32 = -0.5 * PLAYER_BODY_HEIGHT;
const PLAYER_LEG_SHIFT: f32 = -0.5 * (PLAYER_BODY_HEIGHT + PLAYER_LEG_HEIGHT);

const PLAYER_BODY_WIDTH: f32 = 0.3 * PLAYER_HEIGHT;
const PLAYER_LEG_WIDTH: f32 = 0.5 * PLAYER_BODY_WIDTH;
const PLAYER_ARM_WIDTH: f32 = 0.5 * PLAYER_BODY_WIDTH;

const PLAYER_BODY_LENGTH: f32 = 0.25 * PLAYER_BODY_HEIGHT;
const PLAYER_LEG_LENGTH: f32 = PLAYER_BODY_LENGTH;
const PLAYER_ARM_LENGTH: f32 = PLAYER_BODY_LENGTH;

const PLAYER_LEG_WIDTH_SHIFT: f32 = (PLAYER_BODY_WIDTH - PLAYER_LEG_WIDTH) / 2.;
const PLAYER_ARM_WIDTH_SHIFT: f32 = 1.01 * (PLAYER_BODY_WIDTH + PLAYER_ARM_WIDTH) / 2.;

const PLAYER_BODY_SCALE: [f32; 3] = [PLAYER_BODY_LENGTH, PLAYER_BODY_HEIGHT, PLAYER_BODY_WIDTH];
const PLAYER_ARM_SCALE: [f32; 3] = [PLAYER_ARM_LENGTH, PLAYER_ARM_HEIGHT, PLAYER_ARM_WIDTH];
const PLAYER_LEG_SCALE: [f32; 3] = [PLAYER_LEG_LENGTH, PLAYER_LEG_HEIGHT, PLAYER_LEG_WIDTH];

const PLAYER_HEAD_OFFSET: [f32; 3] = [0., PLAYER_HEAD_SIZE / 2., 0.];
const PLAYER_BODY_OFFSET: [f32; 3] = [0., PLAYER_BODY_SHIFT, 0.];
const PLAYER_ARM_OFFSET: [f32; 3] = [0., 0., PLAYER_ARM_WIDTH_SHIFT];
const PLAYER_LEG_OFSET: [f32; 3] = [0., PLAYER_LEG_SHIFT, PLAYER_LEG_WIDTH_SHIFT];

pub type ImageCut = [f32; 4];

/// Define how to cut the image of the player to generate the textures for the player
/// Values are in (u,v) coord, in fraction of the image dimension
pub const PLAYER_CUT_TEMPLATE: [ImageCut; 24] = [
    // Head
    [0., 3. / 4., 1. / 6., 1. / 4.],
    [1. / 6., 3. / 4., 1. / 6., 1. / 4.],
    [2. / 6., 3. / 4., 1. / 6., 1. / 4.],
    [3. / 6., 3. / 4., 1. / 6., 1. / 4.],
    [4. / 6., 3. / 4., 1. / 6., 1. / 4.],
    [5. / 6., 3. / 4., 1. / 6., 1. / 4.],
    // Leg
    [0., 3. / 8., 1. / 12., 3. / 8.],
    [1. / 12., 3. / 8., 1. / 12., 3. / 8.],
    [2. / 12., 3. / 8., 1. / 12., 3. / 8.],
    [3. / 12., 3. / 8., 1. / 12., 3. / 8.],
    [4. / 12., 5. / 8., 1. / 12., 1. / 8.],
    [5. / 12., 5. / 8., 1. / 12., 1. / 8.],
    // Body
    [0., 0., 1. / 12., 3. / 8.],
    [1. / 12., 0., 1. / 6., 3. / 8.],
    [5. / 12., 0., 1. / 12., 3. / 8.],
    [3. / 12., 0., 1. / 6., 3. / 8.],
    [6. / 12., 0., 1. / 6., 1. / 8.],
    [8. / 12., 0., 1. / 6., 1. / 8.],
    // Arm
    [6. / 12., 3. / 8., 1. / 12., 3. / 8.],
    [7. / 12., 3. / 8., 1. / 12., 3. / 8.],
    [8. / 12., 3. / 8., 1. / 12., 3. / 8.],
    [9. / 12., 3. / 8., 1. / 12., 3. / 8.],
    [7. / 12., 1. / 4., 1. / 12., 1. / 8.],
    [8. / 12., 1. / 4., 1. / 12., 1. / 8.],
];

pub const HUMANOID_TEXTURES_PATH: [&str; 2] = ["player.png", "monster.png"];

/// Return a vector of EntityCube forming a humanoid
pub fn get_opengl_entities(mut position: Position, monster_type: u8) -> Vec<EntityCube> {
    let mut ent = Vec::new();

    // Head
    position += Vector3::newf(PLAYER_HEAD_OFFSET).opposite();
    position += Vector3::newf(PLAYER_HEAD_OFFSET)
        .rotation_z(-position.pitch())
        .rotation_y(position.yaw());
    ent.push(EntityCube::new(
        &position,
        0,
        monster_type,
        [PLAYER_HEAD_SIZE; 3],
    ));
    position += Vector3::newf(PLAYER_HEAD_OFFSET)
        .rotation_z(-position.pitch())
        .rotation_y(position.yaw())
        .opposite();

    // Body
    position += Vector3::newf(PLAYER_BODY_OFFSET);
    ent.push(EntityCube::new_only_yaw(
        &position,
        2,
        monster_type,
        PLAYER_BODY_SCALE,
    ));

    // Arm
    position += Vector3::newf(PLAYER_ARM_OFFSET).rotation_y(position.yaw());
    ent.push(EntityCube::new_only_yaw(
        &position,
        3,
        monster_type,
        PLAYER_ARM_SCALE,
    ));
    position += Vector3::newf(PLAYER_ARM_OFFSET).rotation_y(position.yaw()) * -2.;
    ent.push(EntityCube::new_only_yaw(
        &position,
        3,
        monster_type,
        PLAYER_ARM_SCALE,
    ));
    position += Vector3::newf(PLAYER_ARM_OFFSET).rotation_y(position.yaw());

    // Legs
    position += Vector3::newf(PLAYER_LEG_OFSET).rotation_y(position.yaw());
    ent.push(EntityCube::new_only_yaw(
        &position,
        1,
        monster_type,
        PLAYER_LEG_SCALE,
    ));
    position += Vector3::new(0., 0., -2. * PLAYER_LEG_WIDTH_SHIFT).rotation_y(position.yaw());
    ent.push(EntityCube::new_only_yaw(
        &position,
        1,
        monster_type,
        PLAYER_LEG_SCALE,
    ));

    ent
}

/// Returns the bounding box around the player
pub fn humanoid_aabb(eye_position: &Position) -> AABB {
    AABB::new(
        eye_position.z() + DIAMETER / 2.,
        eye_position.z() - DIAMETER / 2.,
        eye_position.y() + FOREHEAD,
        eye_position.y() - PLAYER_HEIGHT + FOREHEAD,
        eye_position.x() + DIAMETER / 2.,
        eye_position.x() - DIAMETER / 2.,
    )
    .unwrap()
}
