use ash::vk;

use winit::event_loop;
use winit::window;

pub struct RendererWindow {
    event_loop: event_loop::EventLoop<()>,
    window: window::Window,
    surface: vk::SurfaceKHR,
    surface_loader: ash::extensions::khr::Surface,
}

impl RendererWindow {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Result<RendererWindow, vk::Result> {
        let event_loop = event_loop::EventLoop::new();
        let window = window::Window::new(&event_loop).unwrap();

        let surface = unsafe {
            ash_window::create_surface(entry, instance, &window, None)?
        };
        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

        Ok(RendererWindow {
            event_loop,
            window,
            surface,
            surface_loader
        })
    }

    pub unsafe fn cleanup(&self) {
        self.surface_loader.destroy_surface(self.surface, None);
    }
}