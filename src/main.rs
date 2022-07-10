pub mod renderer;
pub mod engine;

use engine::Engine;

use anyhow::Result;

fn main() -> Result<()> {
    let mut engine = Engine::new()?;

    engine.run()
}
