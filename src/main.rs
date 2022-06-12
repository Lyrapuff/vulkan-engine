mod renderer;

use renderer::VulkanRenderer;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let _renderer = VulkanRenderer::new()
        .context("Failed to create renderer")?;

    Ok(())
}
