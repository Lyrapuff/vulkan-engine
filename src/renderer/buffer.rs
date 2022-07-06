use ash::vk;
use gpu_allocator::vulkan::*;

use super::allocator::RendererAllocator;

pub struct AllocatedBuffer {
    pub buffer: vk::Buffer,
    pub allocation: Option<Allocation>,
}

impl AllocatedBuffer {
    pub fn cleanup(&mut self, device: &ash::Device, allocator: &mut RendererAllocator) {
        unsafe {
            device.destroy_buffer(self.buffer, None)
        };

        if let Some(allocation) = self.allocation.take() {
            allocator.free(allocation);
        }
    }
}
