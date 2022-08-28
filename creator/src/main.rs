#![deny(clippy::all)]
#![forbid(unsafe_code)]

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

use code_editor::prelude::*;

fn main() -> Result<(), Error> {

    let mut width     : usize = 600;
    let mut height    : usize = 400;

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

    let mut code_editor = CodeEditor::new();
    code_editor.set_font("resources/Source_Code_Pro/static/SourceCodePro-Regular.ttf");
    code_editor.set_mode(CodeEditorMode::Rhai);
    code_editor.set_font_size(17.0);
    code_editor.set_text("testing".to_string());

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
                            if code_editor.key_down(None, Some(WidgetKey::Return)) {
                                window.request_redraw();
                            }
                        }
                        Key::ArrowLeft => {
                            if code_editor.key_down(None, Some(WidgetKey::Left)) {
                                window.request_redraw();
                            }
                        }
                        Key::ArrowRight => {
                            if code_editor.key_down(None, Some(WidgetKey::Right)) {
                                window.request_redraw();
                            }
                        }
                        Key::ArrowUp => {
                            if code_editor.key_down(None, Some(WidgetKey::Up)) {
                                window.request_redraw();
                            }
                        }
                        Key::ArrowDown => {
                            if code_editor.key_down(None, Some(WidgetKey::Down)) {
                                window.request_redraw();
                            }
                        }
                        Key::Backspace => {
                            if code_editor.key_down(None, Some(WidgetKey::Delete)) {
                                window.request_redraw();
                            }
                        }

                        Key::Character(char) => {
                            let chars : Vec<char> = char.chars().collect();
                            if chars.len() > 0 {
                                let char = chars[0];
                                match char {

                                    '\n' => {
                                        if chars.is_empty() == false && code_editor.key_down(None, Some(WidgetKey::Return)) {
                                            window.request_redraw();
                                        }
                                    }
                                    _ => {
                                        if chars.is_empty() == false && code_editor.key_down(Some(chars[0]), None) {
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
                    if code_editor.modifier_changed(m.shift_key(), m.control_key(), m.alt_key(), m.super_key()) {
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
                code_editor.draw(frame, (0, 0, width, height), width);

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
                            if code_editor.mouse_dragged(pixel_pos) {
                                window.request_redraw();
                            }
                        } else
                        if code_editor.mouse_hover(pixel_pos) {
                            window.request_redraw();
                        }
                    }
                }
                DeviceEvent::Button {state, .. } => match state {
                    ElementState::Pressed => {
                        //println!("mouse button {} pressed", button);
                        if let Some(pixel_pos) = pixels.window_pos_to_pixel((coords.x as f32, coords.y as f32)).ok() {
                            is_pressed = true;
                            if code_editor.mouse_down(pixel_pos) {
                                window.request_redraw();
                            }
                        }
                    }
                    ElementState::Released => {
                        //println!("mouse button {} released", button),
                        if let Some(pixel_pos) = pixels.window_pos_to_pixel((coords.x as f32, coords.y as f32)).ok() {
                            is_pressed = false;
                            if code_editor.mouse_up(pixel_pos) {
                                window.request_redraw();
                            }
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
            _ => {}
        }
    });
}
