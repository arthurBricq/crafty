/// Returns the perspective matrix, representing the camera model.
pub fn perspective_matrix(dim: (u32, u32)) -> [[f32; 4]; 4] {
    let (width, height) = dim;
    let aspect_ratio = height as f32 / width as f32;
    let fov: f32 = std::f32::consts::PI / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;
    let f = 1.0 / (fov / 2.0).tan();
    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}
