pub mod debug;
pub mod device;

extern crate core;

use debug::RendererDebug;
use device::RendererDevice;

use ash::prelude::*;
use ash::vk;
use ash::extensions::ext;

use std::ffi;

pub struct VulkanRenderer {
    pub instance: ash::Instance,
    pub debug: RendererDebug,
    pub device: RendererDevice,
}

impl VulkanRenderer {
    pub fn new() -> Result<Self, vk::Result> {
        let entry = ash::Entry::linked();

        let layer_names = vec![
            ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()
        ];

        let layer_pts: Vec<*const i8> = layer_names.iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let instance = Self::create_instance(&entry, &layer_pts)?;

        let debug = RendererDebug::new(&entry, &instance)?;

        let device = match RendererDevice::new(&instance, &[vk::QueueFlags::GRAPHICS], &layer_pts)? {
            None => panic!("No fitting GPU found, don't know what to do"),
            Some(dev) => dev
        };

        let renderer = Self {
            instance,
            debug,
            device,
        };

        Ok(renderer)
    }

    fn create_instance(entry: &ash::Entry, layer_pts: &Vec<*const i8>) -> Result<ash::Instance, vk::Result> {
        let app_name = ffi::CString::new("Vulkan App").unwrap();
        let engine_name = ffi::CString::new("Vulkan Engine").unwrap();

        let extension_name_pts: Vec<*const i8> = vec![
            ext::DebugUtils::name().as_ptr()
        ];

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .engine_name(&engine_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_1);

        let instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_name_pts)
            .enabled_layer_names(layer_pts);

        let instance = unsafe {
            entry.create_instance(&instance_info, None)?
        };

        Ok(instance)
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        unsafe {
            self.device.cleanup();

            self.debug.cleanup();

            self.instance.destroy_instance(None);
        }
    }
}