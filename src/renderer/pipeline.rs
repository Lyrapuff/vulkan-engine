use ash::vk;

use crate::renderer::device::RendererDevice;
use crate::renderer::shader::Shader;
use crate::renderer::mesh::{PushConstantDescription, VertexInputDescription};

use std::ffi;

use anyhow::Result;

pub struct PipelineBuilder<'a> {
    device: &'a RendererDevice,
    extent: vk::Extent2D,
    render_pass: vk::RenderPass,
    shaders: Vec<Shader>,
    bindings: Vec<vk::VertexInputBindingDescription>,
    attributes: Vec<vk::VertexInputAttributeDescription>,
    push_constants: Vec<PushConstantDescription>, 
}

impl<'a> PipelineBuilder<'a> {
    pub fn device(&mut self, device: &'a RendererDevice) -> &mut Self {
        self.device = device;
        self
    }

    pub fn extent(&mut self, extent: vk::Extent2D) -> &mut Self {
        self.extent = extent;
        self
    }

    pub fn render_pass(&mut self, render_pass: vk::RenderPass) -> &mut Self {
        self.render_pass = render_pass;
        self
    }

    pub fn shader(&mut self, shader: Shader) -> &mut Self {
        self.shaders.push(shader);
        self
    }

    pub fn binding(&mut self, binding: vk::VertexInputBindingDescription) -> &mut Self {
        self.bindings.push(binding);
        self
    }

    pub fn bindings(&mut self, bindings: &[vk::VertexInputBindingDescription]) -> &mut Self {
        self.bindings.extend_from_slice(bindings);
        self
    }

    pub fn attribute(&mut self, attribute: vk::VertexInputAttributeDescription) -> &mut Self {
        self.attributes.push(attribute);
        self
    }

    pub fn attributes(&mut self, attributes: &[vk::VertexInputAttributeDescription]) -> &mut Self {
        self.attributes.extend_from_slice(attributes);
        self
    }

    pub fn push_constants(&mut self, push_constants: Vec<PushConstantDescription>) -> &mut Self {
        self.push_constants.extend(push_constants);
        self
    }

    pub fn build(&self) -> Result<RendererPipeline> {
        let entry_point = ffi::CString::new("main").unwrap();

        let mut shader_stages = vec![];

        for shader in &self.shaders {
            shader_stages.push(shader.shader_stage(&entry_point));
        }

        let input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&self.bindings)
            .vertex_attribute_descriptions(&self.attributes);

        let (pipeline_layout, pipeline) = RendererPipeline::create_graphics_pipeline(
            &self.device.logical_device,
            self.render_pass,
            self.extent,
            input_info,
            &shader_stages,
            &self.push_constants,
        )?;

        for shader in &self.shaders {
            unsafe {
                shader.cleanup(&self.device.logical_device);
            }
        }

        Ok(RendererPipeline {
            pipeline_layout,
            pipeline,
        })
    }
}

pub struct RendererPipeline {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
}

impl RendererPipeline {
    pub fn builder(
        device: &RendererDevice,
        extent: vk::Extent2D,
        render_pass: vk::RenderPass,
    ) -> PipelineBuilder {
        PipelineBuilder {
            device,
            extent,
            render_pass,
            shaders: vec![],
            bindings: vec![],
            attributes: vec![],
            push_constants: vec![],
        }
    }

    pub fn create_graphics_pipeline(
        device: &ash::Device,
        render_pass: vk::RenderPass,
        extent: vk::Extent2D,
        vertex_input_info: vk::PipelineVertexInputStateCreateInfoBuilder,
        shader_stages: &[vk::PipelineShaderStageCreateInfo],
        push_constants: &[PushConstantDescription],
    ) -> Result<(vk::PipelineLayout, vk::Pipeline)> {
        // input:

        let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

        // viewport:

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

        // rasterizer:

        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::builder()
            .line_width(1.0)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .cull_mode(vk::CullModeFlags::NONE)
            .polygon_mode(vk::PolygonMode::FILL);

        // multisampler:

        let multisampler_info = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        // color blend:

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

        // push contants:

        let mut push_constant_ranges = vec![];

        for push_constant in push_constants {
           push_constant_ranges.push(vk::PushConstantRange::builder()
               .offset(push_constant.offset)
               .size(push_constant.size)
               .stage_flags(push_constant.stage_flags)
               .build()
           );
        }

        // pipeline layout:

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&push_constant_ranges);

        let pipeline_layout = unsafe {
            device.create_pipeline_layout(&pipeline_layout_info, None)?
        };

        // pipeline:

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(shader_stages)
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
            device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[pipeline_info.build()],
                None,
            ).unwrap()
        }[0];

        Ok((pipeline_layout, pipeline))
    }

    pub unsafe fn cleanup(&self, device: &ash::Device) {
        device.destroy_pipeline(self.pipeline, None);
        device.destroy_pipeline_layout(self.pipeline_layout, None);
    }
}
