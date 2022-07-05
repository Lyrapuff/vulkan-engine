use nalgebra as na;

use crate::renderer::buffer::AllocatedBuffer;

pub struct Vertex {
   pub position: na::Vector3<f32>,
   pub normal: na::Vector3<f32>,
   pub color: na::Vector3<f32>,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: AllocatedBuffer,
}
