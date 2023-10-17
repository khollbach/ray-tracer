use std::result::Result as StdResult;
use std::str::FromStr;

use crate::error::{Error, Result};

use super::Node;

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse(s)
    }
}

// todo: handle errors without crashing
// todo todo: provide helpful error messages
fn parse(text: &str) -> Result<Node> {
    let mut parents = Vec::new();

    // Dummy node, to store top-level nodes.
    parents.push(Node {
        name: "(not a real node)".to_owned(),
        values: vec![],
        children: vec![],
    });

    for mut line in text.lines() {
        // Trim comments.
        if let Some(idx) = line.find("//") {
            line = &line[..idx];
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eyeball_test() {
        let s = "
    // this is a comment
    scene { // this is an end of line comment
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
