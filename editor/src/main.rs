#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod draw2d;
mod ui;
mod context;

pub mod prelude {
    pub use crate::draw2d::Draw2D;
    pub use crate::context::{Context};
    pub use code_editor::WidgetKey;
}

use crate::ui::UI;
use crate::prelude::*;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use tao::{
    dpi::PhysicalPosition,
    dpi::LogicalSize,
    event::{Event, DeviceEvent, ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    menu::{MenuBar, MenuItem},
    window::WindowBuilder,
    keyboard::{Key},
};

fn main() -> Result<(), Error> {

    let mut width     : usize = 800;
    let mut height    : usize = 600;

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let mut file_menu = MenuBar::new();
        file_menu.add_native_item(MenuItem::Quit);

        let mut menu = MenuBar::new();
        menu.add_submenu("File", true, file_menu);

        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("RPU Creator")
            .with_menu(menu)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width as u32, height as u32, surface_texture)?
    };

    // Init the code editor

    let mut ui = UI::new();

    let mut coords = PhysicalPosition::new(0.0, 0.0);
    let mut is_pressed = false;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                // Close events
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::Escape,
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }

                // Resize the window
                WindowEvent::Resized(size) => {
                    pixels.resize_surface(size.width, size.height);
                    let scale = window.scale_factor() as u32;
                    pixels.resize_buffer(size.width / scale, size.height / scale);
                    width = size.width as usize / scale as usize;
                    height = size.height as usize / scale as usize;
                    window.request_redraw();
                }

                WindowEvent::CursorMoved { position, .. } => {
                    coords = position;
                }

                WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                    logical_key: key,
                    state: ElementState::Pressed,
                    ..
                    },
                ..
                } => {
                    // WARNING: Consider using `key_without_modifers()` if available on your platform.
                    // See the `key_binding` example
                    match key {
                        //Key::Escape => *control_flow = ControlFlow::Exit,
                        Key::Enter => {
                            if ui.key_down(None, Some(WidgetKey::Return)) {
                                window.request_redraw();
                            }
                        }
                        Key::ArrowLeft => {
                            if ui.key_down(None, Some(WidgetKey::Left)) {
                                window.request_redraw();
                            }
                        }
                        Key::ArrowRight => {
                            if ui.key_down(None, Some(WidgetKey::Right)) {
                                window.request_redraw();
                            }
                        }
                        Key::ArrowUp => {
                            if ui.key_down(None, Some(WidgetKey::Up)) {
                                window.request_redraw();
                            }
                        }
                        Key::ArrowDown => {
                            if ui.key_down(None, Some(WidgetKey::Down)) {
                                window.request_redraw();
                            }
                        }
                        Key::Backspace => {
                            if ui.key_down(None, Some(WidgetKey::Delete)) {
                                window.request_redraw();
                            }
                        }

                        Key::Character(char) => {
                            let chars : Vec<char> = char.chars().collect();
                            if chars.len() > 0 {
                                let char = chars[0];
                                match char {

                                    '\n' => {
                                        if chars.is_empty() == false && ui.key_down(None, Some(WidgetKey::Return)) {
                                            window.request_redraw();
                                        }
                                    }
                                    _ => {
                                        if chars.is_empty() == false && ui.key_down(Some(chars[0]), None) {
                                            window.request_redraw();
                                        }
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
                WindowEvent::ModifiersChanged(m) => {
                    if ui.modifier_changed(m.shift_key(), m.control_key(), m.alt_key(), m.super_key()) {
                        window.request_redraw();
                    }
                }
                _ => (),
            },

            // Update internal state and request a redraw
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            // Draw the current frame
            Event::RedrawRequested(_) => {

                let frame = pixels.get_frame();
                ui.draw(frame, (0, 0, width, height), width);

                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            },

            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { /*delta,*/ .. } => {
                    //println!("mouse moved: {:?}", delta),
                    if let Some(pixel_pos) = pixels.window_pos_to_pixel((coords.x as f32, coords.y as f32)).ok() {
                        if is_pressed {
                            if ui.mouse_dragged(pixel_pos) {
                                window.request_redraw();
                            }
                        } else
                        if ui.mouse_hover(pixel_pos) {
                            window.request_redraw();
                        }
                    }
                }
                DeviceEvent::Button {state, .. } => match state {
                    ElementState::Pressed => {
                        //println!("mouse button {} pressed", button);
                        if let Some(pixel_pos) = pixels.window_pos_to_pixel((coords.x as f32, coords.y as f32)).ok() {
                            is_pressed = true;
                            if ui.mouse_down(pixel_pos) {
                                window.request_redraw();
                            }
                        }
                    }
                    ElementState::Released => {
                        //println!("mouse button {} released", button),
                        if let Some(pixel_pos) = pixels.window_pos_to_pixel((coords.x as f32, coords.y as f32)).ok() {
                            is_pressed = false;
                            if ui.mouse_up(pixel_pos) {
                                window.request_redraw();
                            }
                        }
                    }
                    _ => (),
                },

                DeviceEvent::MouseWheel { delta, .. } => match delta {
                    // tao::event::MouseScrollDelta::LineDelta(x, y) => {
                    //     println!("mouse wheel Line Delta: ({},{})", x, y);
                    //     let pixels_per_line = 120.0;
                    //     let mut pos = window.outer_position().unwrap();
                    //     pos.x -= (x * pixels_per_line) as i32;
                    //     pos.y -= (y * pixels_per_line) as i32;
                    //     window.set_outer_position(pos)
                    // }
                    tao::event::MouseScrollDelta::PixelDelta(p) => {
                        //println!("mouse wheel Pixel Delta: ({},{})", p.x, p.y);
                        if ui.mouse_wheel((p.x as isize, p.y as isize)) {
                            window.request_redraw();
                            //mouse_wheel_ongoing = true;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
            _ => {}
        }
    });
}
