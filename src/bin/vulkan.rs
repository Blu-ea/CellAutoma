use anyhow::Result;
use cgmath::{vec2, vec3};
use game_life::vulkan::obj::{VERTICES, Vertex, update_vertex_buffer};
use winit::dpi::LogicalSize;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode::{Escape, KeyO, KeyP, KeyR};
use winit::keyboard::PhysicalKey::Code;
use winit::window::{WindowBuilder};

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
    // App

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
            match_key_pressed!(KeyP) => {println!("KeyP");unsafe{VERTICES[0] = Vertex::new(vec2(-0.5, -0.5), vec3(0.0, 1.0, 1.0));update_vertex_buffer(&app.instance, &app.device, &mut app.data);}}
            match_key_pressed!(KeyO) => {println!("KeyO");unsafe{VERTICES[0] = Vertex::new(vec2(-0.5, -0.5), vec3(1.0, 0.0, 0.0));update_vertex_buffer(&app.instance, &app.device, &mut app.data);}}
            // match_key_pressed!(KeyR) => {println!("KeyR");unsafe{let _ = update_vertex_buffer(&app.instance, &app.device, &mut app.data);window.request_redraw();}}
            
            _ => {}
        }
    })?;

    Ok(())
}

#[macro_export]
macro_rules! match_key_pressed {
    ($key:tt) => {
        Event::WindowEvent { event: WindowEvent::KeyboardInput {event: KeyEvent{physical_key: Code($key), state: winit::event::ElementState::Pressed, repeat: false, ..}, ..}, ..}
    };
}