use ash::vk;

use crate::renderer::device::RendererDevice;

use std::ffi;

pub struct RendererPipeline {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
}

impl RendererPipeline {
    pub fn new(device: &RendererDevice, extent: vk::Extent2D, render_pass: vk::RenderPass) -> Result<RendererPipeline, vk::Result> {
        let vert = Self::create_shader_module(device, vk_shader_macros::include_glsl!("./shaders/default.vert"))?;
        let frag = Self::create_shader_module(device, vk_shader_macros::include_glsl!("./shaders/default.frag"))?;

        let entry_point = ffi::CString::new("main").unwrap();

        let shader_stages = [
            Self::shader_stage(&entry_point, vk::ShaderStageFlags::VERTEX, vert),
            Self::shader_stage(&entry_point, vk::ShaderStageFlags::FRAGMENT, frag),
        ];

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder();

        let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

        let viewports = [
            vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: extent.width as f32,
                height: extent.height as f32,
                min_depth: 0.0,
                max_depth: 0.0,
            }
        ];

        let scissors = [
            vk::Rect2D {
                offset: vk::Offset2D {
                    x: 0,
                    y: 0,
                },
                extent,
            }
        ];

        let viewport_info = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::builder()
            .line_width(1.0)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .cull_mode(vk::CullModeFlags::NONE)
            .polygon_mode(vk::PolygonMode::FILL);

        let multisampler_info = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachments = [
            vk::PipelineColorBlendAttachmentState::builder()
                .blend_enable(true)
                .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(vk::BlendFactor::SRC_ALPHA)
                .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .alpha_blend_op(vk::BlendOp::ADD)
                .color_write_mask(
                    vk::ColorComponentFlags::R
                        | vk::ColorComponentFlags::G
                        | vk::ColorComponentFlags::B
                        | vk::ColorComponentFlags::A,
                )
                .build()
        ];

        let color_blend_info = vk::PipelineColorBlendStateCreateInfo::builder()
            .attachments(&color_blend_attachments);

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder();
        let pipeline_layout = unsafe {
            device.device.create_pipeline_layout(&pipeline_layout_info, None)?
        };

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly_info)
            .viewport_state(&viewport_info)
            .rasterization_state(&rasterizer_info)
            .multisample_state(&multisampler_info)
            .color_blend_state(&color_blend_info)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0);

        let pipeline = unsafe {
            device.device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[pipeline_info.build()],
                    None,
            ).unwrap()
        }[0];

        unsafe {
            device.device.destroy_shader_module(vert, None);
            device.device.destroy_shader_module(frag, None);
        }

        Ok(RendererPipeline {
            pipeline,
            pipeline_layout,
        })
    }

    fn create_shader_module(device: &RendererDevice, code: &[u32]) -> Result<vk::ShaderModule, vk::Result> {
        let shader_module_info = vk::ShaderModuleCreateInfo::builder()
            .code(code);

        let shader_module = unsafe {
            device.device.create_shader_module(&shader_module_info, None)?
        };

        Ok(shader_module)
    }

    fn shader_stage(entry_point: &ffi::CString, stage: vk::ShaderStageFlags, module: vk::ShaderModule) -> vk::PipelineShaderStageCreateInfo {
        let stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(stage)
            .module(module)
            .name(entry_point);

        stage.build()
    }

    pub unsafe fn cleanup(&self, device: &ash::Device) {
        device.destroy_pipeline(self.pipeline, None);
        device.destroy_pipeline_layout(self.pipeline_layout, None);
    }
}