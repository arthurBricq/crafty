use crate::texture;
use crate::vertices::cube::{CUBE_FRAGMENT_SHADER, CUBE_VERTEX_SHADER, VERTICES};
use crate::vertices::font::GLChar;
use crate::vertices::rectangle::{RECT_FRAGMENT_SHADER, RECT_VERTEX_SHADER, RECT_VERTICES};
use crate::vertices::{CubeInstance, EntityCube, RectInstance};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{Surface, VertexBuffer, uniform};
use graphics::core::entity::{ENTITY_FRAGMENT_SHADER, ENTITY_VERTEX_SHADER};
use graphics::renderer::{KeyCode, KeyEvent, MouseEvent, RendererBackend, ToDraw, WindowAction};
use primitives::camera::perspective_matrix;
use primitives::color::Color;
use std::time::{Duration, Instant};
use tracing;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::PhysicalKey;
use winit::window::{CursorGrabMode, Fullscreen};

/// 16ms => 60 FPS roughly
const TARGET_FRAME_DURATION: Duration = Duration::from_millis(16);

/// If the frame is `MIN_SLEEP_TIME` shorter than the target duration or less,
/// does not sleep, because of granularity of time in `std::thread::sleep`
const MIN_SLEEP_TIME: Duration = Duration::from_millis(2);

#[derive(Default)]
pub struct GliumRenderer {}

impl graphics::renderer::Renderer for GliumRenderer {
    fn run<B: RendererBackend>(&self, backend: &mut B) {
        // We start by creating the EventLoop, this can only be done once per process.
        // This also needs to happen on the main thread to make the program portable.
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()
            .expect("event loop building");

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Crafty")
            // In macOS, you basically can't resize the window...
            // There is a stretching bug, and I am not sure if it is a `glutin` bug or if it is an implementation bug
            // from my side, due to a different OpenGL implementation on macOS.
            // What I can do for is to create a "big enough" window
            .with_inner_size(3460, 2000)
            .build(&event_loop);

        let lock_mouse = window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked));

        if lock_mouse.is_err() {
            tracing::warn!("Could not lock the mouse")
        }

        #[cfg(not(target_os = "macos"))]
        {
            window.set_cursor_visible(false);
        }

        // Construct the buffer of vertices (for single objects, we use OpenGL's instancing to multiply them)
        let cube_vertex_buffer = VertexBuffer::new(&display, &VERTICES).unwrap();
        let rect_vertex_buffer = VertexBuffer::new(&display, &RECT_VERTICES).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Build the texture library, and change the sampler to use the proper filters
        let textures = texture::build_textures_array(&display);
        let cubes_texture_sampler = textures
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .minify_filter(MinifySamplerFilter::Nearest);

        // Load other textures that are used
        let selected_texture = texture::load_texture(
            std::fs::read("./resources/selected.png")
                .unwrap()
                .as_slice(),
            &display,
        );
        let font_atlas = texture::load_texture(
            std::fs::read("./resources/fonts.png").unwrap().as_slice(),
            &display,
        );

        // Textures for entities
        let humanoid_texture = texture::load_humanoid_textures("./resources/entity/", &display);
        let humanoid_texture_sample = humanoid_texture
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .minify_filter(MinifySamplerFilter::Nearest);

        // Build the shader programs
        let cube_program =
            glium::Program::from_source(&display, CUBE_VERTEX_SHADER, CUBE_FRAGMENT_SHADER, None)
                .unwrap();
        let rect_program =
            glium::Program::from_source(&display, RECT_VERTEX_SHADER, RECT_FRAGMENT_SHADER, None)
                .unwrap();
        let entity_program = glium::Program::from_source(
            &display,
            ENTITY_VERTEX_SHADER,
            ENTITY_FRAGMENT_SHADER,
            None,
        )
        .unwrap();

        // Start rendering by creating a new frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        // Uniform for rect computed before the loop
        let rect_uniforms = uniform! {
            font_atlas: &font_atlas,
            font_offsets: GLChar::get_offset(),
            textures: cubes_texture_sampler
        };

        // Event loop

        let mut t = Instant::now();
        let initial_waiting_delay = Duration::from_secs(1);
        let mut is_initializing = true;

        event_loop
            .run(move |event, window_target| {
                // This block is here to leave some initial time for the world to load updates.
                if is_initializing && t.elapsed() < initial_waiting_delay {
                    return;
                } else if is_initializing {
                    is_initializing = false;
                    t = Instant::now();
                }

                match event {
                    winit::event::Event::WindowEvent { event, .. } => match event {
                        // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                        winit::event::WindowEvent::CloseRequested => window_target.exit(),
                        winit::event::WindowEvent::Resized(tmp) => {
                            tracing::debug!("Resized to {:?}", tmp);
                            backend.set_dimension(display.get_framebuffer_dimensions());
                        }
                        winit::event::WindowEvent::RedrawRequested => {
                            // Step the camera with the elapsed time
                            let dt = t.elapsed();
                            t = Instant::now();

                            let mut target = display.draw();
                            target.clear_color_and_depth(Color::Sky1.to_tuple(), 1.0);

                            // somehow, I have to run a callback on the world-renderer
                            let ToDraw {
                                player_view_matrix,
                                selected_intensity,
                                cubes_buffer,
                                entity_buffer,
                                hud_buffer,
                            } = backend.update(dt);

                            // Convert abstract types to glium vertex types
                            let cubes_buffer: Vec<CubeInstance> = cubes_buffer.iter().map(|d| (*d).into()).collect();
                            let entity_buffer: Vec<EntityCube> = entity_buffer.iter().map(|d| d.clone().into()).collect();
                            let hud_buffer: Vec<RectInstance> = hud_buffer.iter().map(|d| (*d).into()).collect();

                            // I) Draw the cubes

                            // Configure the GPU to do Depth testing (with a depth buffer)
                            let params = glium::DrawParameters {
                                depth: glium::Depth {
                                    test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                                    write: true,
                                    ..Default::default()
                                },
                                blend: glium::draw_parameters::Blend::alpha_blending(),
                                backface_culling:
                                    glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
                                ..Default::default()
                            };

                            // Define our uniforms (same uniforms for all cubes)...
                            let uniforms = uniform! {
                                view: player_view_matrix,
                                perspective: perspective_matrix(target.get_dimensions()),
                                textures: cubes_texture_sampler,
                                selected_texture: &selected_texture,
                                selected_intensity: selected_intensity
                            };

                            // We use OpenGL's instancing feature, which allows us to render huge amounts ot cubes at once.
                            // OpenGL instancing = instead of setting 1000 times different uniforms, you give 1000 attributes once
                            let position_buffer = VertexBuffer::immutable(&display, &cubes_buffer)
                                .expect("Could not create buffer");
                            target
                                .draw(
                                    (&cube_vertex_buffer, position_buffer.per_instance().unwrap()),
                                    &indices,
                                    &cube_program,
                                    &uniforms,
                                    &params,
                                )
                                .unwrap();

                            // II) Draw the entity

                            // Define our uniforms (same uniforms for all cubes)...
                            let entity_uniforms = uniform! {
                                view: player_view_matrix,
                                perspective: perspective_matrix(target.get_dimensions()),
                                entity_textures: humanoid_texture_sample,
                            };

                            // Prepare the entity buffer to send to the gpu
                            let entity_buffer =
                                VertexBuffer::immutable(&display, &entity_buffer).unwrap();
                            target
                                .draw(
                                    (&cube_vertex_buffer, entity_buffer.per_instance().unwrap()),
                                    &indices,
                                    &entity_program,
                                    &entity_uniforms,
                                    &params,
                                )
                                .unwrap();

                            // III) Drawn the tiles

                            // We change the draw parameters here to allow transparency.
                            let draw_parameters = glium::draw_parameters::DrawParameters {
                                blend: glium::draw_parameters::Blend::alpha_blending(),
                                ..glium::draw_parameters::DrawParameters::default()
                            };

                            let rects_buffer =
                                VertexBuffer::dynamic(&display, &hud_buffer).unwrap();
                            target
                                .draw(
                                    (&rect_vertex_buffer, rects_buffer.per_instance().unwrap()),
                                    &indices,
                                    &rect_program,
                                    &rect_uniforms,
                                    &draw_parameters,
                                )
                                .unwrap();
                            target.finish().unwrap();
                        }
                        winit::event::WindowEvent::MouseInput {
                            device_id: _,
                            state,
                            button,
                        } => {
                            // This is essentially a conversion from winit's MouseButton to our own MouseButton
                            // But because of the orphan rule this can't be implemented at the type-level
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
                                // TODO: we should never fail here
                                PhysicalKey::Code(key) => backend.handle_key_event(KeyEvent {
                                    state,
                                    key: winit_keycode_to_custom(key),
                                }),
                                PhysicalKey::Unidentified(_) => vec![],
                            };
                            // Handle the window actions
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
                            let x: f32 =
                                -1. + 2. * position.x as f32 / window.inner_size().width as f32;
                            let y: f32 =
                                1. - 2. * position.y as f32 / window.inner_size().height as f32;
                            backend.cursor_moved(x, y);
                        }
                        _ => (),
                    },
                    winit::event::Event::AboutToWait => {
                        let opt_time_to_sleep = (t + TARGET_FRAME_DURATION - MIN_SLEEP_TIME)
                            .checked_duration_since(Instant::now());
                        if let Some(time_to_sleep) = opt_time_to_sleep {
                            std::thread::sleep(time_to_sleep + MIN_SLEEP_TIME);
                        }
                        window.request_redraw()
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

pub fn winit_keycode_to_custom(winit_key: winit::keyboard::KeyCode) -> KeyCode {
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
        _ => KeyCode::None, // Return None for any KeyCode not supported by your custom enum
    }
}
