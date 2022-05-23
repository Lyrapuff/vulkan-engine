extern crate core;

use ash::prelude::*;
use ash::vk;
use ash::extensions::ext;

use std::ffi;

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();

    println!("[Debug][{}][{}] {:?}", severity, ty, message);

    vk::FALSE
}

struct VulkanRenderer {
    instance: ash::Instance,
    debug: RendererDebug,
    physical_device: vk::PhysicalDevice,
}

impl VulkanRenderer {
    fn new() -> Result<Self, vk::Result> {
        let entry = ash::Entry::linked();

        let instance = Self::create_instance(&entry)?;

        let debug = RendererDebug::new(&entry, &instance)?;

        let physical_device = match Self::pick_physical_device(&instance)? {
            None => panic!("No GPU found, don't know what to do"),
            Some(pd) => pd
        };

        let renderer = Self {
            instance,
            debug,
            physical_device,
        };

        Ok(renderer)
    }

    fn create_instance(entry: &ash::Entry) -> Result<ash::Instance, vk::Result> {
        let app_name = ffi::CString::new("Vulkan App").unwrap();
        let engine_name = ffi::CString::new("Vulkan Engine").unwrap();

        let layer_names = vec![
            ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()
        ];

        let layer_pts: Vec<*const i8> = layer_names.iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

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
            .enabled_layer_names(&layer_pts);

        let instance = unsafe {
            entry.create_instance(&instance_info, None)?
        };

        Ok(instance)
    }

    fn pick_physical_device(instance: &ash::Instance) -> Result<Option<vk::PhysicalDevice>, vk::Result> {
        let physical_devices = unsafe {
            instance.enumerate_physical_devices()?
        };

        let mut chosen = None;

        for physical_device in physical_devices {
            let props: vk::PhysicalDeviceProperties = unsafe {
                instance.get_physical_device_properties(physical_device)
            };

            if props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                chosen = Some(physical_device);

                break;
            }
        }

        Ok(chosen)
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        unsafe {
            self.debug.cleanup();

            self.instance.destroy_instance(None);
        }
    }
}

struct RendererDebug {
    debug_utils: ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl RendererDebug {
    fn new(entry: &ash::Entry, instance: &ash::Instance) -> Result<Self, vk::Result> {
        let debug_utils = ext::DebugUtils::new(entry, instance);

        let messenger_info = vk::DebugUtilsMessengerCreateInfoEXT {
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            pfn_user_callback: Some(vulkan_debug_utils_callback),
            ..Default::default()
        };

        let debug_messenger = unsafe {
            debug_utils.create_debug_utils_messenger(&messenger_info, None)?
        };

        Ok(Self {
            debug_utils,
            debug_messenger
        })
    }

    unsafe fn cleanup(&mut self) {
        self.debug_utils.destroy_debug_utils_messenger(self.debug_messenger, None);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = VulkanRenderer::new()
        .expect("Failed to create renderer");

    Ok(())
}
