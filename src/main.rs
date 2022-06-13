mod renderer;

use renderer::VulkanRenderer;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let renderer = VulkanRenderer::new()
        .context("Failed to create renderer")?;

    renderer.draw();

    Ok(())
}
