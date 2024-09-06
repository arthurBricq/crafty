use glium::{glutin::surface::WindowSurface, Texture2d};
use glium::texture::Texture2dArray;
use glium::Display;
use image::GenericImageView;

use crate::block_kind::Block;
use crate::primitives::math;

pub type ImageCut = [f32; 4];

/// Builds the array of 2D textures using all the blocks
/// Each block is associated with 3 textures: side, top and bottom
/// All these textures are loaded into one single texture array, that is fed to OpenGL.
/// The fragment shader responsible for the cubes is then in charge of selecting the correct element of this array.
pub fn build_textures_array(display: &Display<WindowSurface>) -> Texture2dArray {
    // Get the path of the block textures
    let root = "./resources/block/";
    let extension = ".png";
    let all_textures = Block::get_texture_files();
    let source = all_textures.iter().map(|name| {
        println!(" Adding texture {name} into texture array");
        let data = std::fs::read(root.to_string() + name + extension).unwrap();
        let image = image::load(std::io::Cursor::new(data), image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
    }).collect();
    Texture2dArray::new(display, source).unwrap()
}

/// Loads a texture and returns it
pub fn load_texture(bytes: &[u8], display: &Display<WindowSurface>) -> Texture2d {
    let image = image::load(std::io::Cursor::new(bytes),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    Texture2d::new(display, image).unwrap()
}

/// Loads an image from a path,
/// uses a list of ImageCut to divide the image into sub images,
/// rescales the sub image to a common size and
/// returns a Texture2dArray with these images.
/// ImageCut is in the format \[x, y, height, width\] (u,v) coord,
/// (0, 0) is top left and x, y, height and width are in fraction of the image.
pub fn load_texture_cut(bytes: &[u8], display: &Display<WindowSurface>, cut: &[ImageCut]) -> Texture2dArray {
    let image = image::load(std::io::Cursor::new(bytes),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let mut source = Vec::new();
    // Set a scaling factor which is a common multiplier for every texture
    let mut lcm_x: u32 = 1;
    let mut lcm_y: u32 = 1;
    let dim_x = image.width() as f32;
    let dim_y = image.height() as f32;
    for cut_pos in cut {
        lcm_x = math::lcm(lcm_x, (dim_x * cut_pos[2]) as u32);
        lcm_y = math::lcm(lcm_y, (dim_y * cut_pos[3]) as u32);
    }
    // Load image, rescale it and create a glium texture
    for cut_pos in cut {
        println!("{} {} {} {}", (dim_x * cut_pos[0]) as u32, (dim_y * cut_pos[1]) as u32, (dim_x * cut_pos[2]) as u32, (dim_y * cut_pos[3]) as u32);
        let sub_image = image.view((dim_x * cut_pos[0]) as u32, (dim_y * cut_pos[1]) as u32, (dim_x * cut_pos[2]) as u32, (dim_y * cut_pos[3]) as u32).to_image();
        let sub_image = image::imageops::resize(&sub_image, lcm_x, lcm_y, image::imageops::FilterType::Nearest);
        source.push(glium::texture::RawImage2d::from_raw_rgba_reversed(&sub_image.into_raw(), (lcm_x, lcm_y)));
    }
    Texture2dArray::new(display, source).unwrap()
}