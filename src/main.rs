mod renderer;

use renderer::VulkanRenderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = VulkanRenderer::new()
        .expect("Failed to create renderer");

    Ok(())
}
