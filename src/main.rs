mod color;
mod error;
mod scene;
mod sdl;
mod sphere;
mod vec3;

use std::fs;

use error::Result;
use scene::Scene;

fn main() -> Result<()> {
    let text = fs::read_to_string("scene.sdl")?;
    Scene::from_sdl(&text)?.render();
    Ok(())
}
