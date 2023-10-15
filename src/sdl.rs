//! Scene description language.

mod into_scene;
mod parse;

use crate::error::Result;

#[derive(Debug)]
pub struct Node {
    name: String,
    values: Vec<f64>,
    children: Vec<Node>,
}

impl Node {
    /// `path` is a whitespace-separated string of names.
    ///
    /// Each name must describe exactly one node, or this will fail.
    pub fn get_path(&self, path: &str) -> Result<&Node> {
        let mut curr = self;
        for name in path.split_whitespace() {
            curr = curr
                .children
                .iter()
                .find(|ch| ch.name == name)
                .ok_or_else(|| format!("no child named {name:?}"))?;
        }
        Ok(curr)
    }
}
