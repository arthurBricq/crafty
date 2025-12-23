use image::GenericImageView;
use model::entity::humanoid::{HUMANOID_TEXTURES_PATH, PLAYER_CUT_TEMPLATE};
use model::world::block_kind::Block;
use primitives::math;
use tracing;
use wgpu::{Device, Queue, Texture, TextureFormat, TextureView, TextureViewDescriptor};

/// Builds a 2D array texture containing all block textures
/// Each block has 3 textures: side, top, bottom
pub fn build_block_textures_array(
    device: &Device,
    queue: &Queue,
) -> (Texture, TextureView) {
    let root = "./resources/block/";
    let extension = ".png";
    let all_textures = Block::get_texture_files();
    
    let mut texture_images = Vec::new();
    let mut max_width = 0u32;
    let mut max_height = 0u32;
    
    for name in all_textures {
        tracing::debug!("Adding texture {name} into texture array");
        let data = std::fs::read(format!("{}{}{}", root, name, extension)).unwrap();
        let image = image::load(std::io::Cursor::new(data), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let (width, height) = image.dimensions();
        max_width = max_width.max(width);
        max_height = max_height.max(height);
        texture_images.push(image);
    }
    
    // Resize all textures to the same size
    let mut resized_images = Vec::new();
    for image in texture_images {
        if image.dimensions() != (max_width, max_height) {
            let resized = image::imageops::resize(
                &image,
                max_width,
                max_height,
                image::imageops::FilterType::Nearest,
            );
            resized_images.push(resized);
        } else {
            resized_images.push(image);
        }
    }
    
    // Create texture array
    let array_size = resized_images.len() as u32;
    let texture_size = wgpu::Extent3d {
        width: max_width,
        height: max_height,
        depth_or_array_layers: array_size,
    };
    
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("block_textures_array"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    
    // Upload each texture layer
    for (layer, image) in resized_images.iter().enumerate() {
        let layer_size = wgpu::Extent3d {
            width: max_width,
            height: max_height,
            depth_or_array_layers: 1, // Single layer
        };
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: layer as u32 },
                aspect: wgpu::TextureAspect::All,
            },
            &image.as_raw(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * max_width),
                rows_per_image: Some(max_height),
            },
            layer_size,
        );
    }
    
    let view = texture.create_view(&TextureViewDescriptor {
        label: Some("block_textures_array_view"),
        ..Default::default()
    });
    
    (texture, view)
}

/// Loads a single 2D texture
pub fn load_texture_2d(
    device: &Device,
    queue: &Queue,
    bytes: &[u8],
    label: &str,
) -> (Texture, TextureView) {
    let image = image::load(std::io::Cursor::new(bytes), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();
    let (width, height) = image.dimensions();
    
    let texture_size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    
    let single_layer_size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &image.as_raw(),
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        single_layer_size,
    );
    
    let view = texture.create_view(&TextureViewDescriptor {
        label: Some(&format!("{}_view", label)),
        ..Default::default()
    });
    
    (texture, view)
}

/// Loads entity textures as a 2D array
pub fn load_humanoid_textures(
    device: &Device,
    queue: &Queue,
    root: &str,
) -> (Texture, TextureView) {
    let mut lcm_x: u32 = 1;
    let mut lcm_y: u32 = 1;
    
    // First pass: calculate LCM dimensions
    for name in HUMANOID_TEXTURES_PATH.iter() {
        let data = std::fs::read(format!("{}{}", root, name)).unwrap();
        let image = image::load(std::io::Cursor::new(data), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let dim_x = image.width() as f32;
        let dim_y = image.height() as f32;
        for cut_pos in PLAYER_CUT_TEMPLATE.iter() {
            lcm_x = math::lcm(lcm_x, (dim_x * cut_pos[2]) as u32);
            lcm_y = math::lcm(lcm_y, (dim_y * cut_pos[3]) as u32);
        }
    }
    
    // Second pass: extract and resize sub-images
    let mut texture_array_data = Vec::new();
    
    for name in HUMANOID_TEXTURES_PATH.iter() {
        tracing::debug!("Adding texture {} into texture array", format!("{}{}", root, name));
        let data = std::fs::read(format!("{}{}", root, name)).unwrap();
        let image = image::load(std::io::Cursor::new(data), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        
        let dim_x = image.width() as f32;
        let dim_y = image.height() as f32;
        
        for cut_pos in PLAYER_CUT_TEMPLATE.iter() {
            let sub_image = image.view(
                (dim_x * cut_pos[0]) as u32,
                (dim_y * cut_pos[1]) as u32,
                (dim_x * cut_pos[2]) as u32,
                (dim_y * cut_pos[3]) as u32,
            ).to_image();
            
            let resized = image::imageops::resize(
                &sub_image,
                lcm_x,
                lcm_y,
                image::imageops::FilterType::Nearest,
            );
            texture_array_data.push(resized);
        }
    }
    
    let array_size = texture_array_data.len() as u32;
    let texture_size = wgpu::Extent3d {
        width: lcm_x,
        height: lcm_y,
        depth_or_array_layers: array_size,
    };
    
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("humanoid_textures_array"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    
    // Upload each layer
    for (layer, image) in texture_array_data.iter().enumerate() {
        let layer_size = wgpu::Extent3d {
            width: lcm_x,
            height: lcm_y,
            depth_or_array_layers: 1, // Single layer
        };
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: layer as u32 },
                aspect: wgpu::TextureAspect::All,
            },
            &image.as_raw(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * lcm_x),
                rows_per_image: Some(lcm_y),
            },
            layer_size,
        );
    }
    
    let view = texture.create_view(&TextureViewDescriptor {
        label: Some("humanoid_textures_array_view"),
        ..Default::default()
    });
    
    (texture, view)
}
