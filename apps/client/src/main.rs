use rustcraft_core::camera::Camera;
use rustcraft_render::gpu::Gpu;
use std::{collections::HashSet, sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowId},
};

struct App {
    window: Option<Arc<Window>>,
    gpu: Option<Gpu>,
    camera: Camera,
    keys: HashSet<KeyCode>,
    mouse_captured: bool,
    last_frame: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            gpu: None,
            camera: Camera::default(),
            keys: HashSet::new(),
            mouse_captured: false,
            last_frame: Instant::now(),
        }
    }
}

impl App {
    fn capture_mouse(&mut self, capture: bool) {
        if let Some(window) = &self.window {
            if capture {
                let grabbed = window
                    .set_cursor_grab(CursorGrabMode::Locked)
                    .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined))
                    .is_ok();
                self.mouse_captured = grabbed;
                window.set_cursor_visible(!grabbed);
            } else {
                let _ = window.set_cursor_grab(CursorGrabMode::None);
                window.set_cursor_visible(true);
                self.mouse_captured = false;
            }
        }
    }

    fn update_camera(&mut self) {
        let dt = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
        let down = |key| self.keys.contains(&key) as u8 as f32;
        let forward = down(KeyCode::KeyW) - down(KeyCode::KeyS);
        let right = down(KeyCode::KeyD) - down(KeyCode::KeyA);
        self.camera.move_horizontal(forward, right, dt);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attributes = Window::default_attributes().with_title("Rustcraft");
        let window = Arc::new(event_loop.create_window(attributes).expect("create window"));
        let gpu = pollster::block_on(Gpu::new(Arc::clone(&window))).expect("initialize GPU");
        self.gpu = Some(gpu);
        self.window = Some(window);
        self.last_frame = Instant::now();
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event
            && self.mouse_captured
        {
            self.camera.look(delta.0 as f32, delta.1 as f32);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Focused(false) => {
                self.keys.clear();
                self.capture_mouse(false);
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                ..
            } => self.capture_mouse(true),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    if key == KeyCode::Escape && event.state == ElementState::Pressed {
                        self.capture_mouse(false);
                    } else if event.state == ElementState::Pressed {
                        self.keys.insert(key);
                    } else {
                        self.keys.remove(&key);
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                self.update_camera();
                if let Some(gpu) = &self.gpu {
                    match gpu.render() {
                        Ok(()) => {}
                        Err(wgpu_error) => eprintln!("render error: {wgpu_error}"),
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.run_app(&mut App::default()).expect("run client");
}
