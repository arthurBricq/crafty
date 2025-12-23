use crate::shaders::*;
use crate::texture;
use crate::vertices::*;
use bytemuck::{cast_slice, Pod, Zeroable};
use graphics::renderer::{
    KeyCode, KeyEvent, MouseEvent, Renderer, RendererBackend, ToDraw, WindowAction,
};
use primitives::camera::perspective_matrix;
use primitives::color::Color;
use primitives::font::GLChar;
use std::time::{Duration, Instant};
use wgpu::util::DeviceExt;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::PhysicalKey;
use winit::window::{CursorGrabMode, Fullscreen};

/// 16ms => 60 FPS roughly
const TARGET_FRAME_DURATION: Duration = Duration::from_millis(16);

/// If the frame is `MIN_SLEEP_TIME` shorter than the target duration or less,
/// does not sleep, because of granularity of time in `std::thread::sleep`
const MIN_SLEEP_TIME: Duration = Duration::from_millis(2);

/// Camera uniform buffer structure
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniforms {
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}

/// Font offsets uniform
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct FontOffsets {
    offsets: [f32; 2],
}

/// Fragment uniforms for cube shader
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct FragmentUniforms {
    selected_intensity: f32,
}

/// Converts OpenGL perspective matrix (depth -1..1) to WebGPU perspective matrix (depth 0..1)
/// WebGPU uses depth range 0..1, but the projection matrix calculation is the same
/// The depth range conversion happens automatically in the viewport
fn convert_perspective_matrix(opengl_matrix: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    // The matrix itself is the same, wgpu handles the depth range conversion
    opengl_matrix
}

#[derive(Default)]
pub struct WgpuRenderer {}

impl Renderer for WgpuRenderer {
    fn run<B: RendererBackend>(&self, backend: &mut B) {
        // Create event loop
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()
            .expect("event loop building");

        // Create window
        let window = winit::window::WindowBuilder::new()
            .with_title("Crafty")
            .with_inner_size(winit::dpi::PhysicalSize::new(3460, 2000))
            .build(&event_loop)
            .unwrap();

        let lock_mouse = window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked));

        if lock_mouse.is_err() {
            println!("Could not lock the mouse")
        }

        // #[cfg(not(target_os = "macos"))]
        {
            window.set_cursor_visible(false);
        }

        // Initialize wgpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Use Rc to share window and surface between initialization and closure
        use std::rc::Rc;
        let window_rc = Rc::new(window);
        let window_for_surface = window_rc.clone();
        let surface = Rc::new(instance.create_surface(&*window_for_surface).unwrap());
        let surface_for_closure = surface.clone();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&*surface),
            force_fallback_adapter: false,
        }))
        .expect("Failed to find an appropriate adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))
        .expect("Failed to create device");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let size = window_rc.inner_size();
        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create depth texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Use RefCell to allow mutation inside closure
        use std::cell::RefCell;
        let depth_texture_cell = RefCell::new(depth_texture);
        let depth_view_cell = RefCell::new(depth_view);

        // Load textures
        let (_block_textures, block_textures_view) =
            texture::build_block_textures_array(&device, &queue);
        let (_entity_textures, entity_textures_view) =
            texture::load_humanoid_textures(&device, &queue, "./resources/entity/");
        let (_font_texture, font_texture_view) = texture::load_texture_2d(
            &device,
            &queue,
            &std::fs::read("./resources/fonts.png").unwrap(),
            "font_atlas",
        );
        let (_selected_texture, selected_texture_view) = texture::load_texture_2d(
            &device,
            &queue,
            &std::fs::read("./resources/selected.png").unwrap(),
            "selected_texture",
        );

        // Create samplers
        let nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create shader modules
        let cube_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cube_shader"),
            source: wgpu::ShaderSource::Wgsl(CUBE_VERTEX_SHADER.into()),
        });

        let cube_fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cube_fragment_shader"),
            source: wgpu::ShaderSource::Wgsl(CUBE_FRAGMENT_SHADER.into()),
        });

        let entity_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("entity_shader"),
            source: wgpu::ShaderSource::Wgsl(ENTITY_VERTEX_SHADER.into()),
        });

        let entity_fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("entity_fragment_shader"),
            source: wgpu::ShaderSource::Wgsl(ENTITY_FRAGMENT_SHADER.into()),
        });

        let rect_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("rect_shader"),
            source: wgpu::ShaderSource::Wgsl(RECT_VERTEX_SHADER.into()),
        });

        let rect_fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("rect_fragment_shader"),
            source: wgpu::ShaderSource::Wgsl(RECT_FRAGMENT_SHADER.into()),
        });

        // Create vertex buffers for base meshes
        let cube_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cube_vertex_buffer"),
            contents: cast_slice(&CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let rect_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rect_vertex_buffer"),
            contents: cast_slice(&RECT_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create uniform buffers
        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera_uniform_buffer"),
            size: std::mem::size_of::<CameraUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let font_offsets_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("font_offsets_buffer"),
            contents: cast_slice(&[FontOffsets {
                offsets: GLChar::get_offset(),
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let fragment_uniforms_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("fragment_uniforms_buffer"),
            size: std::mem::size_of::<FragmentUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layouts
        let cube_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("cube_bind_group_layout"),
                entries: &[
                    // Camera uniforms
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Block textures array
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2Array,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Block textures sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Selected texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Selected texture sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Fragment uniforms (selected_intensity)
                    wgpu::BindGroupLayoutEntry {
                        binding: 5,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let entity_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("entity_bind_group_layout"),
                entries: &[
                    // Camera uniforms
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Entity textures array
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2Array,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Entity textures sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let rect_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("rect_bind_group_layout"),
                entries: &[
                    // Font atlas
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Font atlas sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Block textures array (for block icons)
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2Array,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Block textures sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Font offsets uniform
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // Create bind groups
        let cube_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cube_bind_group"),
            layout: &cube_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&block_textures_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&nearest_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&selected_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&nearest_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: fragment_uniforms_buffer.as_entire_binding(),
                },
            ],
        });

        let entity_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("entity_bind_group"),
            layout: &entity_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&entity_textures_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&nearest_sampler),
                },
            ],
        });

        let rect_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rect_bind_group"),
            layout: &rect_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&font_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&nearest_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&block_textures_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&nearest_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: font_offsets_buffer.as_entire_binding(),
                },
            ],
        });

        // Create render pipelines
        let cube_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cube_pipeline_layout"),
            bind_group_layouts: &[&cube_bind_group_layout],
            push_constant_ranges: &[],
        });

        let cube_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("cube_pipeline"),
            layout: Some(&cube_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &cube_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[
                    // Base vertex buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<CubeVertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as u64,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 3]>()
                                    + std::mem::size_of::<[f32; 2]>())
                                    as u64,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Uint32,
                            },
                        ],
                    },
                    // Instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<CubeInstance>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 4]>() as u64,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 2) as u64,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 3) as u64,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 4) as u64,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Uint32,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 4
                                    + std::mem::size_of::<u32>())
                                    as u64,
                                shader_location: 8,
                                format: wgpu::VertexFormat::Uint32,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &cube_fragment_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Culling disabled to match glium behavior
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let entity_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("entity_pipeline_layout"),
                bind_group_layouts: &[&entity_bind_group_layout],
                push_constant_ranges: &[],
            });

        let entity_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("entity_pipeline"),
            layout: Some(&entity_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &entity_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[
                    // Base vertex buffer (same as cube)
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<CubeVertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as u64,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 3]>()
                                    + std::mem::size_of::<[f32; 2]>())
                                    as u64,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Uint32,
                            },
                        ],
                    },
                    // Instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<EntityInstance>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 4]>() as u64,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 2) as u64,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 3) as u64,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 4) as u64,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Uint32,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 4
                                    + std::mem::size_of::<u32>())
                                    as u64,
                                shader_location: 8,
                                format: wgpu::VertexFormat::Uint32,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &entity_fragment_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let rect_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("rect_pipeline_layout"),
            bind_group_layouts: &[&rect_bind_group_layout],
            push_constant_ranges: &[],
        });

        let rect_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rect_pipeline"),
            layout: Some(&rect_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &rect_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[
                    // Base vertex buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<RectVertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as u64,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    },
                    // Instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<RectInstance>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 4]>() as u64,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 2) as u64,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 3) as u64,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 4) as u64,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 5) as u64,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Uint32,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 5
                                    + std::mem::size_of::<u32>())
                                    as u64,
                                shader_location: 8,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 5
                                    + std::mem::size_of::<u32>()
                                    + std::mem::size_of::<[f32; 2]>())
                                    as u64,
                                shader_location: 9,
                                format: wgpu::VertexFormat::Sint32,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &rect_fragment_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None, // No depth testing for UI
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Instance buffers (will be resized as needed)
        let mut cube_instance_buffer: Option<wgpu::Buffer> = None;
        let mut cube_instance_capacity = 0usize;
        let mut entity_instance_buffer: Option<wgpu::Buffer> = None;
        let mut entity_instance_capacity = 0usize;
        let mut rect_instance_buffer: Option<wgpu::Buffer> = None;
        let mut rect_instance_capacity = 0usize;

        // Event loop
        let mut t = Instant::now();
        let initial_waiting_delay = Duration::from_secs(1);
        let mut is_initializing = true;
        let mut is_focused = true; // Track window focus state

        event_loop
            .run(move |event, window_target| {
                let window = &window_rc;
                let surface = &surface_for_closure;
                if is_initializing && t.elapsed() < initial_waiting_delay {
                    return;
                } else if is_initializing {
                    is_initializing = false;
                    t = Instant::now();
                }

                match event {
                    winit::event::Event::WindowEvent { event, .. } => match event {
                        winit::event::WindowEvent::CloseRequested => window_target.exit(),
                        winit::event::WindowEvent::Focused(focused) => {
                            is_focused = focused;
                            if focused {
                                // Window gained focus: re-acquire mouse lock and hide cursor
                                let lock_mouse = window
                                    .set_cursor_grab(CursorGrabMode::Confined)
                                    .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked));
                                if lock_mouse.is_err() {
                                    println!("Could not lock the mouse after regaining focus");
                                }
                                window.set_cursor_visible(false);
                            } else {
                                // Window lost focus: release mouse lock and show cursor
                                let _ = window.set_cursor_grab(CursorGrabMode::None);
                                window.set_cursor_visible(true);
                            }
                        }
                        winit::event::WindowEvent::Resized(new_size) => {
                            config.width = new_size.width.max(1);
                            config.height = new_size.height.max(1);
                            surface.configure(&device, &config);

                            // Recreate depth texture
                            let new_depth_texture =
                                device.create_texture(&wgpu::TextureDescriptor {
                                    label: Some("depth_texture"),
                                    size: wgpu::Extent3d {
                                        width: config.width,
                                        height: config.height,
                                        depth_or_array_layers: 1,
                                    },
                                    mip_level_count: 1,
                                    sample_count: 1,
                                    dimension: wgpu::TextureDimension::D2,
                                    format: wgpu::TextureFormat::Depth32Float,
                                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                                    view_formats: &[],
                                });
                            let new_depth_view = new_depth_texture
                                .create_view(&wgpu::TextureViewDescriptor::default());
                            {
                                let mut depth_tex = depth_texture_cell.borrow_mut();
                                *depth_tex = new_depth_texture;
                            }
                            {
                                let mut depth_v = depth_view_cell.borrow_mut();
                                *depth_v = new_depth_view;
                            }

                            backend.set_dimension((config.width, config.height));
                        }
                        winit::event::WindowEvent::RedrawRequested => {
                            let dt = t.elapsed();
                            t = Instant::now();

                            // Get render data from backend
                            let ToDraw {
                                player_view_matrix,
                                selected_intensity,
                                cubes_buffer,
                                entity_buffer,
                                hud_buffer,
                            } = backend.update(dt);

                            // Convert abstract types to wgpu instance types
                            let cube_instances: Vec<CubeInstance> =
                                cubes_buffer.iter().map(|data| (*data).into()).collect();

                            let entity_instances: Vec<EntityInstance> = entity_buffer
                                .iter()
                                .map(|data| data.clone().into())
                                .collect();

                            let rect_instances: Vec<RectInstance> =
                                hud_buffer.iter().map(|data| (*data).into()).collect();

                            // Update or create instance buffers
                            if cube_instances.len() > cube_instance_capacity {
                                cube_instance_buffer = Some(device.create_buffer_init(
                                    &wgpu::util::BufferInitDescriptor {
                                        label: Some("cube_instance_buffer"),
                                        contents: cast_slice(&cube_instances),
                                        usage: wgpu::BufferUsages::VERTEX
                                            | wgpu::BufferUsages::COPY_DST,
                                    },
                                ));
                                cube_instance_capacity = cube_instances.len();
                            } else if !cube_instances.is_empty() {
                                if let Some(ref buffer) = cube_instance_buffer {
                                    queue.write_buffer(buffer, 0, cast_slice(&cube_instances));
                                }
                            }

                            if entity_instances.len() > entity_instance_capacity {
                                entity_instance_buffer = Some(device.create_buffer_init(
                                    &wgpu::util::BufferInitDescriptor {
                                        label: Some("entity_instance_buffer"),
                                        contents: cast_slice(&entity_instances),
                                        usage: wgpu::BufferUsages::VERTEX
                                            | wgpu::BufferUsages::COPY_DST,
                                    },
                                ));
                                entity_instance_capacity = entity_instances.len();
                            } else if !entity_instances.is_empty() {
                                if let Some(ref buffer) = entity_instance_buffer {
                                    queue.write_buffer(buffer, 0, cast_slice(&entity_instances));
                                }
                            }

                            if rect_instances.len() > rect_instance_capacity {
                                rect_instance_buffer = Some(device.create_buffer_init(
                                    &wgpu::util::BufferInitDescriptor {
                                        label: Some("rect_instance_buffer"),
                                        contents: cast_slice(&rect_instances),
                                        usage: wgpu::BufferUsages::VERTEX
                                            | wgpu::BufferUsages::COPY_DST,
                                    },
                                ));
                                rect_instance_capacity = rect_instances.len();
                            } else if !rect_instances.is_empty() {
                                if let Some(ref buffer) = rect_instance_buffer {
                                    queue.write_buffer(buffer, 0, cast_slice(&rect_instances));
                                }
                            }

                            // Update camera uniforms
                            let proj_matrix = convert_perspective_matrix(perspective_matrix((
                                config.width,
                                config.height,
                            )));
                            let camera_uniforms = CameraUniforms {
                                view: player_view_matrix,
                                projection: proj_matrix,
                            };
                            queue.write_buffer(
                                &camera_uniform_buffer,
                                0,
                                cast_slice(&[camera_uniforms]),
                            );

                            // Update fragment uniforms
                            let fragment_uniforms = FragmentUniforms { selected_intensity };
                            queue.write_buffer(
                                &fragment_uniforms_buffer,
                                0,
                                cast_slice(&[fragment_uniforms]),
                            );

                            // Get surface texture
                            let output = surface.get_current_texture().unwrap();
                            let view = output
                                .texture
                                .create_view(&wgpu::TextureViewDescriptor::default());

                            // Create command encoder
                            let mut encoder =
                                device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                    label: Some("render_encoder"),
                                });

                            {
                                // Render pass for 3D (cubes and entities)
                                let depth_view_borrowed = depth_view_cell.borrow();
                                let mut render_pass =
                                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                        label: Some("3d_render_pass"),
                                        color_attachments: &[Some(
                                            wgpu::RenderPassColorAttachment {
                                                view: &view,
                                                resolve_target: None,
                                                ops: wgpu::Operations {
                                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                                        r: Color::Sky1.to_tuple().0 as f64,
                                                        g: Color::Sky1.to_tuple().1 as f64,
                                                        b: Color::Sky1.to_tuple().2 as f64,
                                                        a: Color::Sky1.to_tuple().3 as f64,
                                                    }),
                                                    store: wgpu::StoreOp::Store,
                                                },
                                            },
                                        )],
                                        depth_stencil_attachment: Some(
                                            wgpu::RenderPassDepthStencilAttachment {
                                                view: &*depth_view_borrowed,
                                                depth_ops: Some(wgpu::Operations {
                                                    load: wgpu::LoadOp::Clear(1.0),
                                                    store: wgpu::StoreOp::Store,
                                                }),
                                                stencil_ops: None,
                                            },
                                        ),
                                        occlusion_query_set: None,
                                        timestamp_writes: None,
                                    });

                                // Draw cubes
                                if !cube_instances.is_empty() {
                                    if let Some(ref cube_instance_buf) = cube_instance_buffer {
                                        render_pass.set_pipeline(&cube_pipeline);
                                        render_pass.set_bind_group(0, &cube_bind_group, &[]);
                                        render_pass
                                            .set_vertex_buffer(0, cube_vertex_buffer.slice(..));
                                        render_pass
                                            .set_vertex_buffer(1, cube_instance_buf.slice(..));
                                        render_pass.draw(0..36, 0..cube_instances.len() as u32);
                                    }
                                }

                                // Draw entities
                                if !entity_instances.is_empty() {
                                    if let Some(ref entity_instance_buf) = entity_instance_buffer {
                                        render_pass.set_pipeline(&entity_pipeline);
                                        render_pass.set_bind_group(0, &entity_bind_group, &[]);
                                        render_pass
                                            .set_vertex_buffer(0, cube_vertex_buffer.slice(..));
                                        render_pass
                                            .set_vertex_buffer(1, entity_instance_buf.slice(..));
                                        render_pass.draw(0..36, 0..entity_instances.len() as u32);
                                    }
                                }
                            }

                            {
                                // Render pass for UI (no depth)
                                let mut render_pass =
                                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                        label: Some("ui_render_pass"),
                                        color_attachments: &[Some(
                                            wgpu::RenderPassColorAttachment {
                                                view: &view,
                                                resolve_target: None,
                                                ops: wgpu::Operations {
                                                    load: wgpu::LoadOp::Load, // Don't clear, draw on top
                                                    store: wgpu::StoreOp::Store,
                                                },
                                            },
                                        )],
                                        depth_stencil_attachment: None,
                                        occlusion_query_set: None,
                                        timestamp_writes: None,
                                    });

                                // Draw UI rectangles
                                if !rect_instances.is_empty() {
                                    if let Some(ref rect_instance_buf) = rect_instance_buffer {
                                        render_pass.set_pipeline(&rect_pipeline);
                                        render_pass.set_bind_group(0, &rect_bind_group, &[]);
                                        render_pass
                                            .set_vertex_buffer(0, rect_vertex_buffer.slice(..));
                                        render_pass
                                            .set_vertex_buffer(1, rect_instance_buf.slice(..));
                                        render_pass.draw(0..6, 0..rect_instances.len() as u32);
                                    }
                                }
                            }

                            // Submit command buffer
                            queue.submit(std::iter::once(encoder.finish()));
                            output.present();
                        }
                        winit::event::WindowEvent::MouseInput {
                            device_id: _,
                            state,
                            button,
                        } => {
                            let button: graphics::renderer::MouseButton = match button {
                                MouseButton::Left => graphics::renderer::MouseButton::Left,
                                MouseButton::Right => graphics::renderer::MouseButton::Right,
                                _ => graphics::renderer::MouseButton::Other,
                            };
                            let state: graphics::renderer::PressedOrReleased = match state {
                                ElementState::Pressed => {
                                    graphics::renderer::PressedOrReleased::Pressed
                                }
                                ElementState::Released => {
                                    graphics::renderer::PressedOrReleased::Released
                                }
                            };
                            backend.handle_mouse_event(MouseEvent { button, state });
                        }
                        winit::event::WindowEvent::KeyboardInput {
                            device_id: _,
                            event,
                            is_synthetic: _,
                        } => {
                            let state: graphics::renderer::PressedOrReleased = match event.state {
                                ElementState::Pressed => {
                                    graphics::renderer::PressedOrReleased::Pressed
                                }
                                ElementState::Released => {
                                    graphics::renderer::PressedOrReleased::Released
                                }
                            };
                            let window_actions = match event.physical_key {
                                PhysicalKey::Code(key) => backend.handle_key_event(KeyEvent {
                                    state,
                                    key: winit_keycode_to_custom(key),
                                }),
                                PhysicalKey::Unidentified(_) => vec![],
                            };
                            for action in window_actions {
                                match action {
                                    WindowAction::SetFullscreen(state) => {
                                        if state {
                                            let monitor_handle =
                                                window.available_monitors().next().unwrap();
                                            window.set_fullscreen(Some(Fullscreen::Borderless(
                                                Some(monitor_handle),
                                            )));
                                        } else {
                                            window.set_fullscreen(None);
                                        }
                                    }
                                    WindowAction::SetCursor(state) => {
                                        window.set_cursor_visible(state);
                                    }
                                }
                            }
                        }
                        winit::event::WindowEvent::CursorMoved { position, .. } => {
                            let window_size = window.inner_size();
                            let x: f32 = -1. + 2. * position.x as f32 / window_size.width as f32;
                            let y: f32 = 1. - 2. * position.y as f32 / window_size.height as f32;
                            backend.cursor_moved(x, y);
                        }
                        _ => (),
                    },
                    winit::event::Event::AboutToWait => {
                        // Only request redraw if window is focused
                        // This reduces CPU/GPU usage when window is in background
                        if is_focused {
                            let opt_time_to_sleep = (t + TARGET_FRAME_DURATION - MIN_SLEEP_TIME)
                                .checked_duration_since(Instant::now());
                            if let Some(time_to_sleep) = opt_time_to_sleep {
                                std::thread::sleep(time_to_sleep + MIN_SLEEP_TIME);
                            }
                            window.request_redraw();
                        }
                    }
                    winit::event::Event::DeviceEvent { event, .. } => match event {
                        winit::event::DeviceEvent::Motion { axis, value } => {
                            backend.handle_motion_event(axis, value);
                        }
                        _ => {}
                    },
                    _ => (),
                };
            })
            .unwrap();
    }
}

fn winit_keycode_to_custom(winit_key: winit::keyboard::KeyCode) -> KeyCode {
    match winit_key {
        winit::keyboard::KeyCode::Escape => KeyCode::Escape,
        winit::keyboard::KeyCode::KeyE => KeyCode::KeyE,
        winit::keyboard::KeyCode::KeyW => KeyCode::KeyW,
        winit::keyboard::KeyCode::KeyS => KeyCode::KeyS,
        winit::keyboard::KeyCode::KeyD => KeyCode::KeyD,
        winit::keyboard::KeyCode::KeyA => KeyCode::KeyA,
        winit::keyboard::KeyCode::KeyK => KeyCode::KeyK,
        winit::keyboard::KeyCode::KeyJ => KeyCode::KeyJ,
        winit::keyboard::KeyCode::Space => KeyCode::Space,
        winit::keyboard::KeyCode::Digit1 => KeyCode::Digit1,
        winit::keyboard::KeyCode::Digit2 => KeyCode::Digit2,
        winit::keyboard::KeyCode::Digit3 => KeyCode::Digit3,
        winit::keyboard::KeyCode::Digit4 => KeyCode::Digit4,
        winit::keyboard::KeyCode::Digit5 => KeyCode::Digit5,
        winit::keyboard::KeyCode::Digit6 => KeyCode::Digit6,
        winit::keyboard::KeyCode::Digit7 => KeyCode::Digit7,
        winit::keyboard::KeyCode::Digit8 => KeyCode::Digit8,
        winit::keyboard::KeyCode::Digit9 => KeyCode::Digit9,
        winit::keyboard::KeyCode::KeyP => KeyCode::KeyP,
        winit::keyboard::KeyCode::KeyX => KeyCode::KeyX,
        winit::keyboard::KeyCode::F3 => KeyCode::F3,
        winit::keyboard::KeyCode::F10 => KeyCode::F10,
        winit::keyboard::KeyCode::F11 => KeyCode::F11,
        winit::keyboard::KeyCode::F12 => KeyCode::F12,
        _ => KeyCode::None,
    }
}
