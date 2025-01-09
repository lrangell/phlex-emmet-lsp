#![allow(unstable_name_collisions)]
use crate::types::EmmetNode;
use itertools::Itertools;
use tracing::info;

pub trait Renderer {
    fn render(&self) -> String;
}
impl Renderer for EmmetNode {
    fn render(&self) -> String {
        let classes = if self.classes.is_empty() {
            None
        } else {
            Some(format!("class: \'{}\'", self.classes.iter().join(" ")))
        };

        let id = self.id.clone().map(|id| format!("id: '{}'", id));

        let arguments: Vec<String> = [classes, id]
            .into_iter()
            .flatten()
            .intersperse(", ".to_owned())
            .collect();

        let args = if arguments.is_empty() {
            "".to_owned()
        } else {
            format!("({})", { arguments.iter().join(", ") })
        };

        format!(
            "{tag}{arguments} {{ {children} }} {siblings}",
            tag = self.tag,
            arguments = args,
            children = self.children.iter().map(|n| n.render()).join("\n"),
            siblings = self.siblings.iter().map(|n| n.render()).join("\n")
        )
    }
}
