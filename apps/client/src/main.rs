use std::sync::Arc;
use rustcraft_render::gpu::Gpu;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() { return; }
        let attributes = Window::default_attributes().with_title("Rustcraft");
        let window = Arc::new(event_loop.create_window(attributes).expect("create window"));
        let gpu = pollster::block_on(Gpu::new(Arc::clone(&window))).expect("initialize GPU");
        self.gpu = Some(gpu);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu { gpu.resize(size.width, size.height); }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = &self.gpu {
                    match gpu.render() {
                        Ok(()) => {},
                        Err(wgpu_error) => eprintln!("render error: {wgpu_error}"),
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(window) = &self.window { window.request_redraw(); }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.run_app(&mut App::default()).expect("run client");
}
