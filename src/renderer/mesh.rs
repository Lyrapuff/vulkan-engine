use nalgebra as na;

use ash::vk;

use crate::renderer::allocator::RendererAllocator;
use crate::renderer::buffer::AllocatedBuffer;

use anyhow::Result;

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

    pub fn upload(&mut self, allocator: &mut RendererAllocator) -> Result<()> {
        let size = (self.vertices.len() * std::mem::size_of::<Vertex>()) as u64;

        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER);

        let buffer = allocator.allocate_buffer(*buffer_info, gpu_allocator::MemoryLocation::CpuToGpu, true)?;

        

        Ok(())
    }
}
