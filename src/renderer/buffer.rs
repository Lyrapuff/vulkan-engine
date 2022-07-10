use ash::vk;
use gpu_allocator::vulkan::*;

use super::allocator::RendererAllocator;

use anyhow::Result;

pub struct AllocatedBuffer {
    pub buffer: vk::Buffer,
    pub allocation: Option<Allocation>,
}

impl AllocatedBuffer {
    pub fn cleanup(&mut self, device: &ash::Device, allocator: &mut RendererAllocator) -> Result<()> {
        unsafe {
            device.destroy_buffer(self.buffer, None)
        };

        if let Some(allocation) = self.allocation.take() {
            allocator.free(allocation)?;
        }

        Ok(())
    }
}
