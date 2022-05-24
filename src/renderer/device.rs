use std::collections::HashMap;
use ash::vk;

pub struct RendererDevice {
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queues: HashMap<vk::QueueFlags, vk::Queue>,
}

impl RendererDevice {
    pub fn new(
        instance: &ash::Instance,
        queue_flags: &[vk::QueueFlags],
        layer_pts: &Vec<*const i8>
    ) -> Result<Option<RendererDevice>, vk::Result> {
        let physical_device = match Self::pick_physical_device(instance)? {
            None => return Ok(None),
            Some(pd) => pd
        };

        let queue_family_props = unsafe {
            instance.get_physical_device_queue_family_properties(physical_device)
        };

        let mut queue_family_indices: HashMap<vk::QueueFlags, u32> = HashMap::new();

        for (i, queue_family) in queue_family_props.iter().enumerate() {
            if queue_family.queue_count <= 0 {
                continue;
            }

            for flag in queue_flags {
                if queue_family.queue_flags.contains(*flag) {
                    queue_family_indices.insert(*flag, i as u32);
                }
            }
        }

        if queue_family_indices.len() != queue_flags.len() {
            return Ok(None);
        }

        let priorities = [1.0f32];

        let mut queue_infos: Vec<vk::DeviceQueueCreateInfo> = Vec::new();

        for (_, i) in queue_family_indices.iter() {
            queue_infos.push(vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities(&priorities)
                .build());
        }

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_layer_names(layer_pts);

        let device = unsafe {
            instance.create_device(physical_device, &device_create_info, None)?
        };

        let mut queues: HashMap<vk::QueueFlags, vk::Queue> = HashMap::new();

        for (flags, i) in queue_family_indices {
            let queue = unsafe {
                device.get_device_queue(i, 0)
            };

            queues.insert(flags, queue);
        }

        Ok(Some(RendererDevice {
            physical_device,
            device,
            queues,
        }))
    }

    fn pick_physical_device(instance: &ash::Instance) -> Result<Option<vk::PhysicalDevice>, vk::Result>  {
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
        self.device.destroy_device(None);
    }
}