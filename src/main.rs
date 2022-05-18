use ash::prelude::*;
use ash::vk;

struct VulkanRenderer {
    instance: ash::Instance,
}

impl VulkanRenderer {
    fn new() -> Result<Self, vk::Result> {
        let entry = ash::Entry::linked();

        let instance = Self::create_instance(&entry)?;

        let renderer = Self {
            instance
        };

        Ok(renderer)
    }

    fn create_instance(entry: &ash::Entry) -> Result<ash::Instance, vk::Result> {
        let instance = unsafe {
            entry.create_instance(&Default::default(), None)?
        };

        Ok(instance)
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = VulkanRenderer::new()
        .expect("Failed to create renderer");

    Ok(())
}
