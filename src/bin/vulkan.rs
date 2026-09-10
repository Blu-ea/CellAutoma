use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::DeviceEvent::MouseMotion;
use winit::event::{ElementState, Event, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode::{Escape, KeyA, KeyD, KeyE, KeyQ, KeyS, KeyW, ShiftLeft, Space};
use winit::keyboard::PhysicalKey::Code;
use winit::window::{CursorGrabMode, WindowBuilder};

use game_life::vulkan::App;


fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;
    let mut minimized = false;
    let mut cursor_grabed = false;

    let mut app = unsafe { App::create(&window)? };

    event_loop.run(move |event, elwt| {
        match event {
            Event::AboutToWait => window.request_redraw(),

            Event::WindowEvent { event: WindowEvent::Resized(size), .. } =>{
                if size.width == 0 || size.height == 0 {
                    minimized = true;
                } else {
                    minimized = false;
                    app.resized = true;
                }
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested , .. } if !elwt.exiting() && !minimized => unsafe { app.render(&window) }.unwrap(),

            Event::WindowEvent { event: WindowEvent::CloseRequested , .. } | match_key_pressed!(Escape) => {
                elwt.exit();
                unsafe { app.destroy(); }
            }

            match_key_pressed!(KeyE) => { println!("Camera pos - {:?}", app.camera.pos) }
            match_key_pressed!(KeyQ) => { println!("Control - {:?}", app.control) }

            // match_key_pressed!(KeyR) => {println!("KeyR");unsafe{let _ = update_vertex_buffer(&app.instance, &app.device, &mut app.data);window.request_redraw();}}
            Event::WindowEvent { event: WindowEvent::KeyboardInput{ .. }, .. } => {
                bind_movement_keys!(event, app.control, {
                    Space     => up,
                    ShiftLeft => down,
                    KeyW      => forward,
                    KeyS      => back,
                    KeyA      => left,
                    KeyD      => right,
                })
            }

            Event::WindowEvent {event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left , .. }, .. } => {
                match cursor_grabed{
                    true => {if let Err(err) = window.set_cursor_grab(CursorGrabMode::None){
                                println!("Error {err:?}");
                            } else {
                                window.set_cursor_visible(true);
                                cursor_grabed = false 
                            }
                        }
                    false => {if let Err(err) = window.set_cursor_grab(CursorGrabMode::Locked){
                                println!("Error {err:?}");
                            } else {
                                window.set_cursor_visible(false);
                                cursor_grabed = true 
                            }
                        }
                }
                println!("Cursor {:?}", if cursor_grabed {"Locked"} else {"None"})
            }

            Event::DeviceEvent { event: MouseMotion{delta} , ..} => {
                // if !cursor_grabed {return}
                let sensitivity = 0.002;
                app.camera.yaw -= delta.0 as f32 * sensitivity;
                app.camera.pitch -= delta.1 as f32 * sensitivity;

                app.camera.pitch = app.camera.pitch.clamp(
                    -std::f32::consts::FRAC_PI_2 + 0.01,
                    std::f32::consts::FRAC_PI_2 - 0.01,
                );
            }

            _ => {}
        }
    })?;

    Ok(())
}

#[macro_export]
macro_rules! match_key_pressed {
    ($key:pat) => {
        Event::WindowEvent { event: WindowEvent::KeyboardInput {event: KeyEvent{physical_key: Code($key), state: winit::event::ElementState::Pressed, repeat: false, ..}, ..}, ..}
    };
}

#[macro_export]
macro_rules! match_key_released {
    ($key:pat) => {
        Event::WindowEvent { event: WindowEvent::KeyboardInput {event: KeyEvent{physical_key: Code($key), state: winit::event::ElementState::Released, repeat: false, ..}, ..}, ..}
    };
}

#[macro_export]
macro_rules! bind_movement_keys {
    ($event:expr, $control:expr, { $($key:tt => $field:ident),* $(,)? }) => {
        match $event {
            $(
                match_key_pressed!($key) => { $control.$field = true; }
                match_key_released!($key) => { $control.$field = false; }
            )*
            _ => {}
        }
    };
}