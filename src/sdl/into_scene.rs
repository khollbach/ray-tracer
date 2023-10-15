// todo: these conversion methods seem repetitive and error prone
// What to do about it?
// Maybe tests would be a good start, but the code could be better too.

// todo: refactor f64 -> uX conversion code dup

use crate::{
    color::Color,
    error::{Error, Result},
    sphere::Sphere,
    vec3::Vec3,
};

use super::Node;

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
