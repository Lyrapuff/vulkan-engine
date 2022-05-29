use ash::vk;

use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct RendererWindow {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: ash::extensions::khr::Surface,
}

impl RendererWindow {
    pub fn create_window() -> Result<(EventLoop<()>, Window), winit::error::OsError> {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop)?;

        Ok((event_loop, window))
    }

    pub fn new(
        event_loop: EventLoop<()>,
        window: Window,
        entry: &ash::Entry,
        instance: &ash::Instance
    ) -> Result<RendererWindow, vk::Result> {
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

    pub fn capabilities(
        &self,
        physical_device: vk::PhysicalDevice
    ) -> Result<vk::SurfaceCapabilitiesKHR, vk::Result> {
        unsafe {
            self.surface_loader.get_physical_device_surface_capabilities(physical_device, self.surface)
        }
    }

    pub fn formats(
        &self,
        physical_device: vk::PhysicalDevice
    ) -> Result<Vec<vk::SurfaceFormatKHR>, vk::Result> {
        unsafe {
            self.surface_loader.get_physical_device_surface_formats(physical_device, self.surface)
        }
    }
}