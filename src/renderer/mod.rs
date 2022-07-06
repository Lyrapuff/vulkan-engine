pub mod debug;
pub mod device;
pub mod window;
pub mod swapchain;
pub mod pipeline;
pub mod shader;
pub mod command_pools;
pub mod allocator;
pub mod mesh;
pub mod buffer;

use debug::RendererDebug;
use device::RendererDevice;
use window::RendererWindow;
use swapchain::RendererSwapchain;
use pipeline::RendererPipeline;
use command_pools::CommandPools;
use allocator::RendererAllocator;
use mesh::Mesh;
use mesh::Vertex;

use ash::vk;
use ash::extensions::ext;
use ash::extensions::khr;

use std::ffi;

use nalgebra as na;

use anyhow::Result;

pub struct VulkanRenderer {
    pub instance: ash::Instance,
    pub debug: RendererDebug,
    pub main_device: RendererDevice,
    pub window: RendererWindow,
    pub swapchain: RendererSwapchain,
    pub render_pass: vk::RenderPass,
    pub graphics_pipeline: RendererPipeline,
    pub command_pools: CommandPools,
    pub graphics_command_buffers: Vec<vk::CommandBuffer>,
    pub allocator: RendererAllocator,
    pub triangle_mesh: Mesh,
}

impl VulkanRenderer {
    fn used_layer_names() -> Vec<ffi::CString> {
        vec![
            ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()
        ]
    }

    fn used_extensions() -> Vec<*const i8> {
        vec![
            ext::DebugUtils::name().as_ptr(),
            khr::Surface::name().as_ptr(),
        ]
    }

    pub fn new() -> Result<Self> {
        let (event_loop, window) = RendererWindow::create_window()?;

        window.set_title("Vulkan Engine");

        let used_layer_names = Self::used_layer_names();

        let used_layers = used_layer_names.iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let mut used_extensions = Self::used_extensions();

        for ext_name in ash_window::enumerate_required_extensions(&window)? {
            used_extensions.push(*ext_name);
        }

        let entry = ash::Entry::linked();

        let instance = Self::create_instance(&entry, &used_layers, &used_extensions)?;

        let window = RendererWindow::new(event_loop, window, &entry, &instance)?;

        let debug = RendererDebug::new(&entry, &instance)?;

        let main_device = match RendererDevice::new(&instance, &used_layers)? {
            None => panic!("No fitting GPU found, don't know what to do"),
            Some(dev) => dev
        };

        let render_pass = Self::create_render_pass(&main_device, &window)?;

        let mut swapchain = RendererSwapchain::new(&instance, &main_device, &window)?;
        swapchain.create_framebuffers(&main_device, render_pass)?;

        let graphics_pipeline = RendererPipeline::new(&main_device, swapchain.extent, render_pass)?;

        let command_pools = CommandPools::new(&main_device)?;

        let graphics_command_buffers = CommandPools::create_command_buffers(
            &main_device,
            command_pools.graphics,
            swapchain.framebuffers.len() as u32
        )?;

        let allocator_desc = gpu_allocator::vulkan::AllocatorCreateDesc {
           instance: instance.clone(), 
           device: main_device.logical_device.clone(),
           physical_device: main_device.physical_device,
           debug_settings: Default::default(),
           buffer_device_address: true,
        };

        let allocator = RendererAllocator::new(&allocator_desc)?;

        let mut triangle = Mesh::empty();

        triangle.vertices.push(Vertex::with(
            na::Vector3::new(1.0, 1.0, 0.0),
            na::Vector3::identity(),
            na::Vector3::new(0.0, 1.0, 0.0),
        ));

        triangle.vertices.push(Vertex::with(
            na::Vector3::new(-1.0, 1.0, 0.0),
            na::Vector3::identity(),
            na::Vector3::new(0.0, 1.0, 0.0),
        ));

        triangle.vertices.push(Vertex::with(
            na::Vector3::new(0.0, -1.0, 0.0),
            na::Vector3::identity(),
            na::Vector3::new(0.0, 1.0, 0.0),
        ));

        let renderer = Self {
            instance,
            debug,
            main_device,
            window,
            swapchain,
            render_pass,
            graphics_pipeline,
            command_pools,
            graphics_command_buffers,
            allocator,
            triangle_mesh: triangle,
        };

        renderer.fill_command_buffers()?;

        Ok(renderer)
    }

    fn create_instance(
        entry: &ash::Entry,
        layer_name_pts: &Vec<*const i8>,
        extension_name_pts: &Vec<*const i8>
    ) -> Result<ash::Instance> {
        let app_name = ffi::CString::new("Vulkan App")?;
        let engine_name = ffi::CString::new("Vulkan Engine")?;

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .engine_name(&engine_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_1);

        let instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(extension_name_pts)
            .enabled_layer_names(layer_name_pts);

        let instance = unsafe {
            entry.create_instance(&instance_info, None)?
        };

        Ok(instance)
    }

    fn create_render_pass(device: &RendererDevice, window: &RendererWindow) -> Result<vk::RenderPass> {
        let formats = window.formats(device.physical_device)?;
        let format = formats.first().unwrap();

        let attachments = [
            vk::AttachmentDescription::builder()
                .format(format.format)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .samples(vk::SampleCountFlags::TYPE_1)
                .build()
        ];

        let color_attachment_references = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let subpasses = [
            vk::SubpassDescription::builder()
                .color_attachments(&color_attachment_references)
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .build()
        ];

        let subpass_dependencies = [
            vk::SubpassDependency::builder()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_subpass(0)
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .build()
        ];

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&subpass_dependencies);

        let render_pass = unsafe {
            device.logical_device.create_render_pass(&render_pass_info, None)?
        };

        Ok(render_pass)
    }

    fn fill_command_buffers(&self) -> Result<()> {
        for (i, &command_buffer) in self.graphics_command_buffers.iter().enumerate() {
            let begin_info = vk::CommandBufferBeginInfo::builder();

            unsafe {
                self.main_device.logical_device.begin_command_buffer(command_buffer, &begin_info)?
            };

            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    }
                },
            ];

            let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.render_pass)
                .framebuffer(self.swapchain.framebuffers[i])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: self.swapchain.extent,
                })
                .clear_values(&clear_values);

            unsafe {
                self.main_device.logical_device.cmd_begin_render_pass(
                    command_buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE,
                );

                self.main_device.logical_device.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.graphics_pipeline.pipeline,
                );

                self.main_device.logical_device.cmd_draw(command_buffer, 3, 1, 0, 0);

                self.main_device.logical_device.cmd_end_render_pass(command_buffer);

                self.main_device.logical_device.end_command_buffer(command_buffer)?;
            };
        }

        Ok(())
    }
}

impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        self.allocator.cleanup();

        unsafe {
            self.main_device.logical_device.device_wait_idle().unwrap();

            self.command_pools.cleanup(&self.main_device);

            self.graphics_pipeline.cleanup(&self.main_device.logical_device);

            self.main_device.logical_device.destroy_render_pass(self.render_pass, None);

            self.swapchain.cleanup(&self.main_device);

            self.window.cleanup();

            self.main_device.cleanup();

            self.debug.cleanup();

            self.instance.destroy_instance(None);
        }
    }
}
