use ash::vk;

use std::ffi;

pub struct Shader {
    pub shader_module: vk::ShaderModule,
    pub stage: vk::ShaderStageFlags,
}

impl Shader {
    pub fn from_code(device: &ash::Device, code: &[u32], stage: vk::ShaderStageFlags) -> Result<Shader, vk::Result> {
        let shader_module_info = vk::ShaderModuleCreateInfo::builder()
            .code(code);

        let shader_module = unsafe {
            device.create_shader_module(&shader_module_info, None)?
        };

        Ok(Shader {
            shader_module,
            stage,
        })
    }

    pub fn from_code_vert(device: &ash::Device, code: &[u32]) -> Result<Shader, vk::Result> {
        Self::from_code(device, code, vk::ShaderStageFlags::VERTEX)
    }

    pub fn from_code_frag(device: &ash::Device, code: &[u32]) -> Result<Shader, vk::Result> {
        Self::from_code(device, code, vk::ShaderStageFlags::FRAGMENT)
    }

    pub fn shader_stage(&self, entry_point: &ffi::CString) -> vk::PipelineShaderStageCreateInfo {
        let stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(self.stage)
            .module(self.shader_module)
            .name(entry_point);

        stage.build()
    }

    pub unsafe fn cleanup(&self, device: &ash::Device) {
        device.destroy_shader_module(self.shader_module, None);
    }
}
