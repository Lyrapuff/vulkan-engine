use ash::vk;
use ash::extensions::khr;

use crate::renderer::device::RendererDevice;
use crate::renderer::window::RendererWindow;

use anyhow::Result;

pub struct RendererSwapchain {
    pub swapchain_loader: khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub image_views: Vec<vk::ImageView>,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub extent: vk::Extent2D,
}

impl RendererSwapchain {
    pub fn new(
        instance: &ash::Instance,
        device: &RendererDevice,
        window: &RendererWindow
    ) -> Result<RendererSwapchain> {
        // swapchain creation:

        let graphics_queue_family = match device.queue_family(vk::QueueFlags::GRAPHICS) {
            None => panic!("No graphics queue family found, don't know what to do!"),
            Some(qf) => qf
        };

        let queue_families = [graphics_queue_family.index];

        let capabilities = window.capabilities(device.physical_device)?;

        let formats = window.formats(device.physical_device)?;
        let format = formats.first().unwrap();

        let (swapchain_loader, swapchain) = Self::create_swapchain(
            window.surface,
            &capabilities,
            format,
            &queue_families,
            instance,
            device,
        )?;

        // swapchain images:

        let images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)?
        };

        let image_views = Self::create_image_views(&images, &device)?;

        Ok(RendererSwapchain {
            swapchain_loader,
            swapchain,
            image_views,
            framebuffers: vec![],
            extent: capabilities.current_extent,
        })
    }

    fn create_swapchain(
        surface: vk::SurfaceKHR,
        capabilities: &vk::SurfaceCapabilitiesKHR,
        format: &vk::SurfaceFormatKHR,
        queue_families: &[u32],
        instance: &ash::Instance,
        device: &RendererDevice,
    ) -> Result<(khr::Swapchain, vk::SwapchainKHR)> {
        let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(3.max(capabilities.min_image_count).min(capabilities.max_image_count))
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(capabilities.current_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .queue_family_indices(&queue_families)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO);

        let swapchain_loader = khr::Swapchain::new(instance, &device.logical_device);
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&swapchain_info, None)?
        };

        Ok((swapchain_loader, swapchain))
    }

    fn create_image_views(images: &Vec<vk::Image>, device: &RendererDevice) -> Result<Vec<vk::ImageView>> {
        let mut image_views = Vec::with_capacity(images.len());

        for image in images {
            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);

            let image_view_info = vk::ImageViewCreateInfo::builder()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(vk::Format::B8G8R8A8_UNORM)
                .subresource_range(*subresource_range);

            let image_view = unsafe {
                device.logical_device.create_image_view(&image_view_info, None)?
            };

            image_views.push(image_view);
        }

        Ok(image_views)
    }

    pub fn create_framebuffers(&mut self, device: &RendererDevice, render_pass: vk::RenderPass) -> Result<()> {
        for image_view in &self.image_views {
            let image_view = [*image_view];

            let framebuffer_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(&image_view)
                .width(self.extent.width)
                .height(self.extent.height)
                .layers(1);

            let framebuffer = unsafe {
                device.logical_device.create_framebuffer(&framebuffer_info, None)?
            };

            self.framebuffers.push(framebuffer);
        }

        Ok(())
    }

    pub unsafe fn cleanup(&self, device: &RendererDevice) {
        for framebuffer in &self.framebuffers {
            device.logical_device.destroy_framebuffer(*framebuffer, None);
        }

        for image_view in &self.image_views {
            device.logical_device.destroy_image_view(*image_view, None);
        }

        self.swapchain_loader.destroy_swapchain(self.swapchain, None);
    }
}