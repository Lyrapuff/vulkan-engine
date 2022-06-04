use ash::vk;
use ash::extensions::khr;

use crate::renderer::device::RendererDevice;
use crate::renderer::window::RendererWindow;

pub struct RendererSwapchain {
    pub swapchain_loader: khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub image_views: Vec<vk::ImageView>,
}

impl RendererSwapchain {
    pub fn new(
        instance: &ash::Instance,
        device: &RendererDevice,
        window: &RendererWindow
    ) -> Result<RendererSwapchain, vk::Result> {
        // swapchain creation:

        let queue_families = [device.graphics_queue_family];

        let capabilities = window.capabilities(device.physical_device)?;
        let formats = window.formats(device.physical_device)?;

        let format = formats.first().unwrap();

        let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(window.surface)
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

        let swapchain_loader = khr::Swapchain::new(instance, &device.device);
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&swapchain_info, None)?
        };

        // swapchain images:

        let images = unsafe {
            swapchain_loader.get_swapchain_images(swapchain)?
        };

        let mut image_views = Vec::with_capacity(images.len());

        for image in &images {
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
                device.device.create_image_view(&image_view_info, None)?
            };

            image_views.push(image_view);
        }

        Ok(RendererSwapchain {
            swapchain_loader,
            swapchain,
            image_views,
        })
    }

    pub unsafe fn cleanup(&self, device: &RendererDevice) {
        for image_view in &self.image_views {
            device.device.destroy_image_view(*image_view, None);
        }

        self.swapchain_loader.destroy_swapchain(self.swapchain, None);
    }
}