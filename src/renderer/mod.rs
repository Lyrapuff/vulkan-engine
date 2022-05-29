pub mod debug;
pub mod device;
pub mod window;
pub mod swapchain;

extern crate core;

use debug::RendererDebug;
use device::RendererDevice;
use window::RendererWindow;
use swapchain::RendererSwapchain;

use ash::vk;
use ash::extensions::ext;

use std::ffi;

pub struct VulkanRenderer {
    pub instance: ash::Instance,
    pub debug: RendererDebug,
    pub main_device: RendererDevice,
    pub window: RendererWindow,
    pub swapchain: RendererSwapchain,
}

impl VulkanRenderer {
    fn used_layer_names() -> Vec<ffi::CString> {
        vec![
            ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()
        ]
    }

    fn used_extensions() -> Vec<*const i8> {
        vec![
            ext::DebugUtils::name().as_ptr(),
            ash::extensions::khr::Surface::name().as_ptr(),
        ]
    }

    pub fn new() -> Result<Self, vk::Result> {
        let (event_loop, window) = RendererWindow::create_window()
            .expect("Failed to create window");

        window.set_title("Vulkan Engine");

        let used_layer_names = Self::used_layer_names();

        let used_layers = used_layer_names.iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let mut used_extensions = Self::used_extensions();

        for ext_name in ash_window::enumerate_required_extensions(&window)? {
            used_extensions.push(ext_name.as_ptr());
        }

        let entry = ash::Entry::linked();

        let instance = Self::create_instance(&entry, &used_layers, &used_extensions)?;

        let window = RendererWindow::new(event_loop, window, &entry, &instance)?;

        let debug = RendererDebug::new(&entry, &instance)?;

        let main_device = match RendererDevice::new(&instance, &used_layers)? {
            None => panic!("No fitting GPU found, don't know what to do"),
            Some(dev) => dev
        };

        let swapchain = RendererSwapchain::new(&instance, &main_device, &window)?;

        let renderer = Self {
            instance,
            debug,
            main_device,
            window,
            swapchain,
        };

        Ok(renderer)
    }

    fn create_instance(
        entry: &ash::Entry,
        layer_name_pts: &Vec<*const i8>,
        extension_name_pts: &Vec<*const i8>
    ) -> Result<ash::Instance, vk::Result> {
        let app_name = ffi::CString::new("Vulkan App").unwrap();
        let engine_name = ffi::CString::new("Vulkan Engine").unwrap();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .engine_name(&engine_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_1);

        let instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(extension_name_pts)
            .enabled_layer_names(layer_name_pts);

        let instance = unsafe {
            entry.create_instance(&instance_info, None)?
        };

        Ok(instance)
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        unsafe {
            self.swapchain.cleanup(&self.main_device);

            self.window.cleanup();

            self.main_device.cleanup();

            self.debug.cleanup();

            self.instance.destroy_instance(None);
        }
    }
}