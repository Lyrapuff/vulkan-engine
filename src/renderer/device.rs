use ash::vk;

use anyhow::Result;

pub struct RendererDevice {
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: ash::Device,
    pub graphics_queue_family: u32,
    pub graphics_queue: vk::Queue,
}

impl RendererDevice {
    fn used_extensions() -> Vec<*const i8> {
        vec![
            ash::extensions::khr::Swapchain::name().as_ptr()
        ]
    }

    pub fn new(
        instance: &ash::Instance,
        layer_pts: &Vec<*const i8>,
    ) -> Result<Option<RendererDevice>> {
        let physical_device = match Self::pick_physical_device(instance)? {
            None => return Ok(None),
            Some(pd) => pd
        };

        let queue_family_props = unsafe {
            instance.get_physical_device_queue_family_properties(physical_device)
        };

        let mut graphics_queue_found = None;

        for (i, queue_family) in queue_family_props.iter().enumerate() {
            if queue_family.queue_count <= 0 {
                continue;
            }

            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                graphics_queue_found = Some(i as u32);

                break;
            }
        }

        let graphics_queue_family = match graphics_queue_found {
            None => return Ok(None),
            Some(i) => i
        };

        let priorities = [1.0f32];

        let queue_infos: Vec<vk::DeviceQueueCreateInfo> = vec![
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(graphics_queue_family)
                .queue_priorities(&priorities)
                .build()
        ];

        let used_extensions = Self::used_extensions();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&used_extensions)
            .enabled_layer_names(layer_pts);

        let device = unsafe {
            instance.create_device(physical_device, &device_create_info, None)?
        };

        let graphics_queue = unsafe {
            device.get_device_queue(graphics_queue_family, 0)
        };

        Ok(Some(RendererDevice {
            physical_device,
            logical_device: device,
            graphics_queue,
            graphics_queue_family,
        }))
    }

    fn pick_physical_device(instance: &ash::Instance) -> Result<Option<vk::PhysicalDevice>>  {
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

    pub unsafe fn cleanup(&self) {
        self.logical_device.destroy_device(None);
    }
}