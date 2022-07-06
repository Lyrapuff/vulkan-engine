use gpu_allocator::vulkan::*;

use ash::vk;

use crate::renderer::buffer::AllocatedBuffer;

use anyhow::Result;

pub struct RendererAllocator {
    allocator: std::mem::ManuallyDrop<Allocator>,
    device: ash::Device,
}

impl RendererAllocator {
    pub fn new(desc: &AllocatorCreateDesc) -> Result<Self> {
        let allocator = Allocator::new(desc)?;

        Ok(Self {
            allocator: std::mem::ManuallyDrop::new(allocator),
            device: desc.device.clone(),
        })
    }

    pub fn free(&mut self, allocation: Allocation) -> Result<()> {
        self.allocator.free(allocation)?;

        Ok(())
    }

    pub fn allocate_buffer(
        &mut self,
        buffer_info: vk::BufferCreateInfo,
        location: gpu_allocator::MemoryLocation,
        linear: bool,
    ) -> Result<AllocatedBuffer> {
        let buffer = unsafe {
            self.device.create_buffer(&buffer_info, None)
        }?;

        let requirements = unsafe {
            self.device.get_buffer_memory_requirements(buffer)
        };

        let allocation_info = AllocationCreateDesc {
           name: "rab",
           requirements,
           location,
           linear,
        };

        let allocation = self.allocator.allocate(&allocation_info)?;

        unsafe {
            self.device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())?
        };

        Ok(AllocatedBuffer {
            buffer,
            allocation: Some(allocation),
        })
    }

    pub fn cleanup(&mut self) {
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.allocator);
        }
    }
}
