use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
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
            Event::WindowEvent { event: WindowEvent::CloseRequested , .. } => {
                elwt.exit();
                unsafe { app.destroy(); }
            }

            _ => {}
        }
    })?;

    Ok(())
}
