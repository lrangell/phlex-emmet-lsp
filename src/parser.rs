use anyhow::anyhow;
use anyhow::Result;

use crate::types::EmmetNode;

peg::parser! {
  grammar emmet() for str {
    rule identifier() -> String
            = n:$(quiet!{[^ '#' | '.' | '*' | '{' |  '+' |  '>' ]+}) {n.to_string()} / expected!("identifier")

    rule id() -> String = "#" i:identifier() {i}

    rule class() -> String = "." c:identifier() {c}

    rule tag() -> String = t:identifier() {t}

    rule multiplier() -> usize
            = "*" n:$(['0'..='9']+) {n.parse().unwrap()}

    rule text() -> String
        = "{" chars:([^'}']*) "}" { chars.iter().collect::<String>() }

    rule children() -> EmmetNode
            = ">" n:node()  { n }
    rule sibling() -> EmmetNode
            = "+" n:node()  { n }

    rule implicit_tag() -> String
        = &(id() / class() / ) { "div".to_string() }

    pub rule node() -> EmmetNode
            = tag:(tag() / implicit_tag())
              id:id()?
              classes:class()*
              text:text()?
              multiplier:multiplier()?
              children:children()*
              siblings:sibling()*
            {
                EmmetNode {
                    tag,
                    id,
                    classes,
                    text,
                    multiplier: multiplier.unwrap_or(1),
                    children,
                    siblings
                }
            }


  }
}

pub fn parse(input: &str) -> Result<EmmetNode> {
    emmet::node(input).map_err(|e| anyhow!("Failed to parse Emmet expression: {}", e))
}
