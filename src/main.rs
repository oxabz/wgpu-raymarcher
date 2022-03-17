mod app;
mod camera;
mod shapes;
pub mod color;

use winit::event::{ElementState, Event, VirtualKeyCode};
use winit::event::WindowEvent;
use winit::event::KeyboardInput;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::event_loop::ControlFlow;
use crate::app::AppState;


async fn run(event_loop: EventLoop<()>, window:Window) {
    let mut app = AppState::new(&window).await;
    let mut last_frame = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow|{
        match event {
            // Only handle window event
            Event::WindowEvent {event, window_id,..}if window_id == window.id()  => {
                match event {
                    // Input handled by application so do nothing
                    event if app.input(&event) => {}
                    // Stop the loop if the application is required to stop
                    WindowEvent::CloseRequested |
                    WindowEvent::Destroyed |
                    WindowEvent::KeyboardInput { input:KeyboardInput {state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..} => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input: winit::event::KeyboardInput{ virtual_keycode:Some(winit::event::VirtualKeyCode::R), ..},.. } => {
                        window.request_redraw();
                    }
                    // Handle resizing
                    WindowEvent::Resized(size) =>{
                        app.resize(size);
                        window.request_redraw();
                    }
                    WindowEvent::ScaleFactorChanged{new_inner_size, ..} =>{
                        app.resize(*new_inner_size);
                        window.request_redraw();
                    }
                    _ => {}
                }
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let delta_t = std::time::Instant::now()-last_frame;
                last_frame=std::time::Instant::now();
                app.update(delta_t);
                match app.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::RedrawEventsCleared | Event::MainEventsCleared => {
                let delta_t = std::time::Instant::now()-last_frame;

                if delta_t.as_millis()<500 {
                    window.request_redraw();
                }
            }
            // Any other event is ignore
            _ => {}
        }
    })
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    env_logger::init();

    pollster::block_on(run(event_loop, window));
}
