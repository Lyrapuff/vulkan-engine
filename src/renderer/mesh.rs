use nalgebra as na;

use ash::vk;
use winit::platform::unix::x11::ffi::XF86XK_PowerOff;

use crate::renderer::allocator::RendererAllocator;
use crate::renderer::buffer::AllocatedBuffer;

use anyhow::Result;

pub struct VertexInputDescription {
    pub bindings: Vec<vk::VertexInputBindingDescription>,
    pub attributes: Vec<vk::VertexInputAttributeDescription>,
    pub flags: vk::PipelineVertexInputStateCreateFlags,
}

#[repr(C)]
pub struct Vertex {
   pub position: na::Vector3<f32>,
   pub normal: na::Vector3<f32>,
   pub color: na::Vector3<f32>,
}

impl Vertex {
    pub fn with(position: na::Vector3<f32>, normal: na::Vector3<f32>, color: na::Vector3<f32>) -> Self {
        Vertex {
            position,
            normal,
            color,
        }
    }

    pub fn input_description() -> VertexInputDescription {
        let mut bindings = Vec::new();
        let mut attributes = Vec::new();

        bindings.push(vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(std::mem::size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
        );

        attributes.push(vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(memoffset::offset_of!(Self, position) as u32)
            .build()
        );

        attributes.push(vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(memoffset::offset_of!(Self, normal) as u32)
            .build()
        );

        attributes.push(vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(memoffset::offset_of!(Self, color) as u32)
            .build()
        );

        VertexInputDescription {
            bindings,
            attributes,
            flags: vk::PipelineVertexInputStateCreateFlags::default(),
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: Option<AllocatedBuffer>,
}

impl Mesh {
    pub fn empty() -> Self {
        Self {
            vertices: vec![],
            vertex_buffer: None,
        }
    }

    pub fn upload(&mut self, device: &ash::Device, allocator: &mut RendererAllocator) -> Result<()> {
        if let Some(buffer) = &mut self.vertex_buffer.take() {
            buffer.cleanup(device, allocator);
        }

        let size = (self.vertices.len() * std::mem::size_of::<Vertex>()) as u64;

        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER);

        let buffer = allocator.allocate_buffer(*buffer_info, gpu_allocator::MemoryLocation::CpuToGpu, true)?;

        if let Some(allocation) = &buffer.allocation {
            let data_ptr = allocation.mapped_ptr().unwrap().as_ptr() as *mut Vertex;

            unsafe {
                data_ptr.copy_from_nonoverlapping(self.vertices.as_ptr(), self.vertices.len());
            }
        }

        self.vertex_buffer = Some(buffer);

        Ok(())
    }

    pub fn cleanup(&mut self, device: &ash::Device, allocator: &mut RendererAllocator) {
        if let Some(buffer) = &mut self.vertex_buffer {
            buffer.cleanup(device, allocator);
        }
    }
}
