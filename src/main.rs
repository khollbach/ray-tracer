mod color;
mod error;
mod objects;
mod scene;
mod sdl;
mod vec3;

use std::{fs, env};

use error::Result;
use scene::Scene;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    if args.len() != 1 {
        Err("expected 1 argument: <scene-file.sdl>")?
    }
    let filename = args.next().unwrap();
    let text = fs::read_to_string(filename)?;
    Scene::from_sdl(&text)?.render();
    Ok(())
}
