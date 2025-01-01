use anyhow::anyhow;
use anyhow::Result;

use crate::types::EmmetNode;

peg::parser! {
  grammar emmet() for str {
    rule identifier() -> String
            = n:$(quiet!{['a'..='z' | 'A'..='Z' | '0'..='9' | '-' ]+}) {n.to_string()} / expected!("identifier")

    rule id() -> String = "#" i:identifier() {i}

    rule class() -> String = "." c:identifier() {c}

    rule tag() -> String = t:identifier() {t}

    rule multiplier() -> usize
            = "*" n:$(['0'..='9']+) {n.parse().unwrap()}

    rule text() -> String = "{" t:$([_]+) "}" { t.to_string() }


    rule children() -> EmmetNode
            = ">" n:node()  { n }
    rule sibling() -> EmmetNode
            = "+" n:node()  { n }

    pub rule node() -> EmmetNode
            = tag:tag() id:id()? classes:class()* multiplier:multiplier()? children:children()* siblings:sibling()*
                { EmmetNode { tag, id, classes, multiplier: multiplier.unwrap_or(1), children , siblings } }

  }
}

pub fn parse(input: &str) -> Result<EmmetNode> {
    emmet::node(input).map_err(|e| anyhow!("Failed to parse Emmet expression: {}", e))
}
