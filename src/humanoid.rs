use crate::graphics::entity::EntityCube;
use crate::vector::Vector3;

// Define some constants to draw a player
// Height taken from human proportion in drawing
const PLAYER_HEIGHT: f32 = 1.0;
const PLAYER_HEAD_SIZE: f32 = 0.170 * PLAYER_HEIGHT;
const PLAYER_BODY_HEIGHT: f32 = 0.370 * PLAYER_HEIGHT;
const PLAYER_LEG_HEIGHT: f32 = 0.460 * PLAYER_HEIGHT;
const PLAYER_ARM_HEIGHT: f32 = 0.370 * PLAYER_HEIGHT;

const PLAYER_BODY_SHIFT: f32 = - 0.5 * ( PLAYER_HEAD_SIZE + PLAYER_BODY_HEIGHT );
const PLAYER_LEG_SHIFT: f32 = - 0.5 * ( PLAYER_BODY_HEIGHT  + PLAYER_LEG_HEIGHT );

const PLAYER_BODY_WIDTH: f32 = 0.3 * PLAYER_HEIGHT;
const PLAYER_LEG_WIDTH: f32 = 0.5 * PLAYER_BODY_WIDTH;
const PLAYER_ARM_WIDTH: f32 = 0.5 * PLAYER_BODY_WIDTH;

const PLAYER_BODY_LENGTH: f32 = 0.25 * PLAYER_BODY_HEIGHT;
const PLAYER_LEG_LENGTH: f32 = PLAYER_BODY_LENGTH;
const PLAYER_ARM_LENGTH: f32 = PLAYER_BODY_LENGTH;

const PLAYER_LEG_WIDTH_SHIFT: f32 = (PLAYER_BODY_WIDTH - PLAYER_LEG_WIDTH) / 2.;
const PLAYER_ARM_WIDTH_SHIFT: f32 = 1.01 * (PLAYER_BODY_WIDTH + PLAYER_ARM_WIDTH ) / 2.;

const PLAYER_BODY_SCALE: [f32; 3] = [PLAYER_BODY_LENGTH, PLAYER_BODY_HEIGHT, PLAYER_BODY_WIDTH];
const PLAYER_ARM_SCALE: [f32; 3] = [PLAYER_ARM_LENGTH, PLAYER_ARM_HEIGHT, PLAYER_ARM_WIDTH];
const PLAYER_LEG_SCALE: [f32; 3] = [PLAYER_LEG_LENGTH, PLAYER_LEG_HEIGHT, PLAYER_LEG_WIDTH];

// Define the position of the textures to use for the player
pub const PATRON_PLAYER_CUT: [[u32; 4]; 24] = [
    // Head
    [0, 24, 8, 8], [8, 24, 8, 8], [16, 24, 8, 8], [24, 24, 8, 8], [32, 24, 8, 8], [40, 24, 8, 8], 
    // Leg
    [0, 12, 4, 12], [4, 12, 4, 12], [8, 12, 4, 12], [12, 12, 4, 12], [16, 20, 4, 4], [20, 20, 4, 4],
    // Body
    [0, 0, 4, 12], [4, 0, 8, 12], [20, 0, 4, 12], [12, 0, 8, 12], [24, 0, 8, 4], [32, 0, 8, 4], 
    // Arm
    [24, 12, 4, 12], [28, 12, 4, 12], [32, 12, 4, 12], [36, 12, 4, 12], [28, 8, 4, 4], [32, 8, 4, 4],
];


pub fn draw(position: Vector3, rot: [f32; 2]) -> Vec<EntityCube> {

    let mut position = position;
    let mut ent = Vec::new();

    // Head
    ent.push(EntityCube::new(&position, 0, [PLAYER_HEAD_SIZE; 3], rot ));
    // Body
    position += Vector3::new(0., PLAYER_BODY_SHIFT, 0.);
    ent.push(EntityCube::new(&position, 2, PLAYER_BODY_SCALE, [rot[0], 0.]));
    // Arm
    position += Vector3::new(0., 0., PLAYER_ARM_WIDTH_SHIFT).rotation_y(rot[0]);
    ent.push(EntityCube::new(&position, 3, PLAYER_ARM_SCALE, [rot[0], 0.]));
    position += Vector3::new(0., 0., -2. * PLAYER_ARM_WIDTH_SHIFT).rotation_y(rot[0]);
    ent.push(EntityCube::new(&position, 3, PLAYER_ARM_SCALE, [rot[0], 0.]));
    position += Vector3::new(0., 0., PLAYER_ARM_WIDTH_SHIFT).rotation_y(rot[0]);
        // Leg
    position += Vector3::new(0., PLAYER_LEG_SHIFT, PLAYER_LEG_WIDTH_SHIFT).rotation_y(rot[0]);
    ent.push(EntityCube::new(&position, 1, PLAYER_LEG_SCALE, [rot[0], 0.0]));
    position += Vector3::new(0., 0., -2. * PLAYER_LEG_WIDTH_SHIFT).rotation_y(rot[0]);
    ent.push(EntityCube::new(&position, 1, PLAYER_LEG_SCALE, [rot[0], 0.0 ]));
    
    ent
}