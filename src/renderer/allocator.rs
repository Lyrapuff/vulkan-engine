use gpu_allocator::vulkan::*;

use anyhow::Result;

pub struct RendererAllocator {
    allocator: Allocator,
    device: ash::Device,
}

impl RendererAllocator {
    pub fn new(desc: &AllocatorCreateDesc) -> Result<Self> {
        let allocator = Allocator::new(desc)?;

        Ok(Self {
            allocator,
            device: desc.device.clone(),
        })
    }
}
