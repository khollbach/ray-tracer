//! Scene description language.

use crate::{
    color::Color,
    error::{Error, Result},
    sphere::Sphere,
    vec3::Vec3,
};
use std::result::Result as StdResult;

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

// todo: these conversion methods seem repetitive and error prone
// What to do about it?
// Maybe tests would be a good start, but the code could be better too.

// todo: refactor f64 -> uX conversion code dup

impl TryFrom<&Node> for f64 {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Self> {
        if !node.children.is_empty() {
            Err(format!(
                "cannot convert to f64. node has {} children (expected 0)",
                node.children.len()
            ))?
        }
        if node.values.len() != 1 {
            Err(format!(
                "cannot convert to f64. node has {} values (expected 1)",
                node.values.len()
            ))?
        }
        Ok(node.values[0])
    }
}

impl TryFrom<&Node> for u32 {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Self> {
        if !node.children.is_empty() {
            Err(format!(
                "cannot convert to u32. node has {} children (expected 0)",
                node.children.len()
            ))?
        }
        if node.values.len() != 1 {
            Err(format!(
                "cannot convert to u32. node has {} values (expected 1)",
                node.values.len()
            ))?
        }
        let value = node.values[0];
        if value.fract() != 0. {
            Err(format!(
                "cannot convert to u32. value has fractional component: {}",
                value
            ))?
        }
        if value.clamp(0., u32::MAX as f64) != value {
            Err(format!(
                "cannot convert to u32. value out of range for u32: {}",
                value
            ))?
        }
        Ok(value as u32)
    }
}

impl TryFrom<&Node> for Vec3 {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Self> {
        if !node.children.is_empty() {
            Err(format!(
                "cannot convert to Vec3. node has {} children (expected 0)",
                node.children.len()
            ))?
        }
        if node.values.len() != 3 {
            Err(format!(
                "cannot convert to Vec3. node has {} values (expected 3)",
                node.values.len()
            ))?
        }
        Ok(Vec3::new(node.values[0], node.values[1], node.values[2]))
    }
}

impl TryFrom<&Node> for Color {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Self> {
        if !node.children.is_empty() {
            Err(format!(
                "cannot convert to Color. node has {} children (expected 0)",
                node.children.len()
            ))?
        }
        if node.values.len() != 3 {
            Err(format!(
                "cannot convert to Color. node has {} values (expected 3)",
                node.values.len()
            ))?
        }
        for i in 0..3 {
            if node.values[i].fract() != 0. {
                Err(format!(
                    "cannot convert to Color. node has fractional value {}",
                    node.values[i]
                ))?
            }
            if node.values[i].clamp(0., 255.) != node.values[i] {
                Err(format!(
                    "cannot convert to Color. node value out of range 0-255: {}",
                    node.values[i]
                ))?
            }
        }
        Ok(Color::new(
            node.values[0] as u8,
            node.values[1] as u8,
            node.values[2] as u8,
        ))
    }
}

impl TryFrom<&Node> for Vec<Sphere> {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Self> {
        let fail = "cannot convert to Vec<Sphere>:";
        if !node.values.is_empty() {
            Err(format!(
                "{fail} node has {} values (expected 0)",
                node.values.len()
            ))?
        }
        node.children
            .iter()
            .map(TryFrom::try_from)
            .collect::<Result<Self>>()
            .map_err(|e| format!("{fail} {e}").into())
    }
}

impl TryFrom<&Node> for Sphere {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Self> {
        let fail = "cannot convert to Sphere:";
        if !node.values.is_empty() {
            Err(format!(
                "{fail} node has {} values (expected 0)",
                node.values.len()
            ))?
        }
        Ok(Self {
            color: node.get_path("color")?.try_into()?,
            center: node.get_path("center")?.try_into()?,
            radius: node.get_path("radius")?.try_into()?,
        })
    }
}

// todo: handle errors without crashing
// todo todo: provide helpful error messages
pub fn parse(text: &str) -> Result<Node> {
    let mut parents = Vec::new();

    // Dummy node, to store top-level nodes.
    parents.push(Node {
        name: "(not a real node)".to_owned(),
        values: vec![],
        children: vec![],
    });

    for line in text.lines() {
        let mut words: Vec<_> = line.split_whitespace().collect();
        if words.is_empty() {
            continue; // ignore blank lines
        }
        if words[0] == "}" {
            // close this context
            let curr = parents.pop().unwrap();
            let containing = parents.last_mut().unwrap();
            containing.children.push(curr);
            continue;
        }

        if !is_valid_name(words[0]) {
            Err(format!("invalid node name: {}", words[0]))?
        }
        let name = words[0].to_owned();

        let has_children = words.len() > 1 && *words.last().unwrap() == "{";
        if has_children {
            words.pop();
        }

        let values = words[1..]
            .iter()
            .map(|s| s.parse())
            .collect::<StdResult<_, _>>()?;

        let node = Node {
            name,
            values,
            children: vec![],
        };

        if has_children {
            // enter a new context
            parents.push(node);
        } else {
            parents.last_mut().unwrap().children.push(node);
        }
    }

    // only the dummy context should be un-closed
    assert_eq!(parents.len(), 1);

    // there should be exactly one top-level node in the file
    assert_eq!(parents[0].children.len(), 1);

    Ok(parents.pop().unwrap().children.pop().unwrap())
}

fn is_valid_name(name: &str) -> bool {
    let valid_char = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    !name.starts_with(|c: char| c.is_ascii_digit()) && name.chars().all(valid_char)
}

#[test]
mod tests {
    #[test]
    fn eyeball_test() {
        let s = "
    scene {
        camera {
            position 0 0 -20
            up 0 1 0
            right 1 0 0
        }
        focal-distance 10
        screen {
            width 64
            height 48
        }
        lights {
            light {
                position -10 10 -20
                color 255 200 255
            }
        }
        objects {
            sphere {
                color 0 255 0
                center -5 2.5 -2
                radius 10
            }
            sphere {
                color 0 0 255
                center 5 -2.5 5
                radius 15
            }
        }
    }
        ";

        // looks good to me
        dbg!(parse(s).unwrap());
    }
}
