use ash::vk;
use gpu_allocator::vulkan::*;

use super::allocator::RendererAllocator;

pub struct AllocatedBuffer {
    pub buffer: vk::Buffer,
    pub allocation: Allocation,
}

impl AllocatedBuffer {

}
