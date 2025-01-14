#![allow(unstable_name_collisions)]
use crate::types::EmmetNode;
use itertools::Itertools;

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

        let siblings = format!("\n{}", self.siblings.iter().map(|n| n.render()).join("\n"));
        let children = self.children.iter().map(|n| n.render()).join("\n");

        let expansion = format!("{tag}{args} {{ {children} }}{siblings}", tag = self.tag,);

        match self.multiplier {
            1 => expansion,
            n => [expansion.as_str()].repeat(n).join("\n"),
        }
    }
}
