use ash::vk;

use anyhow::Result;

pub struct QueueFamily {
    pub index: u32,
    pub flags: vk::QueueFlags,
    pub queues: Vec<vk::Queue>,
}

pub struct RendererDevice {
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: ash::Device,
    pub queue_families: Vec<QueueFamily>,
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

        let mut queue_families = Self::pick_queue_families(instance, physical_device);

        let priorities = [1.0f32];

        let mut queue_infos: Vec<vk::DeviceQueueCreateInfo> = Vec::with_capacity(queue_families.len());

        for queue_family in &queue_families {
            queue_infos.push(
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(queue_family.index)
                    .queue_priorities(&priorities)
                    .build()
            )
        }

        let used_extensions = Self::used_extensions();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&used_extensions)
            .enabled_layer_names(layer_pts);

        let device = unsafe {
            instance.create_device(physical_device, &device_create_info, None)?
        };

        for queue_family in &mut queue_families {
            unsafe {
                queue_family.queues.push(device.get_device_queue(queue_family.index, 0));
            };
        }

        Ok(Some(RendererDevice {
            physical_device,
            logical_device: device,
            queue_families,
        }))
    }

    pub fn queue_family(&self, flags: vk::QueueFlags) -> Option<&QueueFamily> {
        for queue_family in &self.queue_families {
            if queue_family.flags == flags {
                return Some(queue_family)
            }
        }

        None
    }

    fn pick_physical_device(
        instance: &ash::Instance
    ) -> Result<Option<vk::PhysicalDevice>>  {
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

    fn pick_queue_families(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice
    ) -> Vec<QueueFamily> {
        let queue_family_props = unsafe {
            instance.get_physical_device_queue_family_properties(physical_device)
        };

        let mut queue_families: Vec<QueueFamily> = Vec::new();

        for (i, queue_family) in queue_family_props.iter().enumerate() {
            if queue_family.queue_count <= 0 {
                continue;
            }

            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                queue_families.push(QueueFamily {
                    index: i as u32,
                    flags: vk::QueueFlags::GRAPHICS,
                    queues: vec![],
                });

                break;
            }
        }

        queue_families
    }

    pub unsafe fn cleanup(&self) {
        self.logical_device.destroy_device(None);
    }
}