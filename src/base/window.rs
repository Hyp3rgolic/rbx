use crate::base::dev::*;
use glam::{Mat4, Vec2, Vec3};
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("RBX")
        .build(&event_loop)
        .unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let mut size = window.inner_size();
    let mut aspect = size.width as f32 / size.height as f32;
    let mut buffer = Vec::new();
    let mut meshes = vec![Mesh::new()];
    let mut pointer = Vec2::new(0.0, 0.0);
    let mut camera = Camera {
        pos: Vec3::new(0.0, 0.0, 10.0),
        dir: Vec3::new(0.0, 0.0, -1.0),
        up: Vec3::Z,
        fov: 70.0,
        near: 0.1,
        far: 100.0,
    };

    buffer.resize(size.width as usize * size.height as usize, 0);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let proj_mat = Mat4::from_cols_array(&[
                    camera.fov.to_radians() / aspect,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    camera.fov.to_radians(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    camera.far / (camera.far - camera.near),
                    (-camera.far * camera.near) - (camera.far - camera.near),
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                ]);
                let view_mat = Mat4::look_at_lh(camera.pos, camera.pos + camera.dir, camera.up);

                clear(&mut buffer);
                draw_pixel(&mut buffer, &size, pointer, 0x00ffffff);

                render(&mut buffer, &size, &meshes, proj_mat, view_mat);

                surface.set_buffer(&buffer, size.width as u16, size.height as u16);
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::Resized(_) => {
                    size = window.inner_size();
                    aspect = size.width as f32 / size.height as f32;
                    if buffer.len() != size.width as usize * size.height as usize {
                        buffer.resize(size.width as usize * size.height as usize, 0);
                    }
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Space),
                            ..
                        },
                    ..
                } => {
                    window.request_redraw();
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::A),
                            ..
                        },
                    ..
                } => {
                    camera.dir.x += 1.0;
                    window.request_redraw();
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::W),
                            ..
                        },
                    ..
                } => {
                    camera.dir.z += 1.0;
                    window.request_redraw();
                }

                WindowEvent::CursorMoved { .. } => {
                    window
                        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                        .unwrap();
                }

                _ => {}
            },

            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    pointer.x += dbg!(delta.0) as f32;
                    pointer.y += delta.1 as f32;
                    window.request_redraw();
                }
                _ => {}
            },

            _ => {}
        }
    });
}
