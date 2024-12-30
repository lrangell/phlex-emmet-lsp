use anyhow::anyhow;
use anyhow::Result;

use nom::{
    bytes::complete::is_not,
    character::complete::{alphanumeric1, char, digit1},
    combinator::opt,
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

use crate::types::EmmetNode;
use nom_supreme::error::ErrorTree;

fn parse_emmet_node(input: &str) -> IResult<&str, EmmetNode, ErrorTree<&str>> {
    let class = many0(preceded(char('.'), is_not(".#>+")));
    let id = opt(preceded(char('#'), is_not(".#>+")));
    let multiplier = opt(preceded(char('*'), digit1));

    let mut single_node = tuple((alphanumeric1, id, class, multiplier));
    let (input, (tag, id, classes, multi)) = single_node(input)?;
    Ok((
        input,
        EmmetNode {
            tag: tag.to_owned(),
            id: id.map(str::to_string),
            classes: classes.iter().map(|s| s.to_string()).collect(),
            children: Vec::new(),
            siblings: Vec::new(),
            multiplier: multi.unwrap_or("1").parse::<usize>().unwrap_or(1),
        },
    ))
}

fn parse_children(input: &str) -> IResult<&str, Vec<EmmetNode>, ErrorTree<&str>> {
    many0(preceded(char('>'), parse_emmet))(input)
}

fn parse_siblings(input: &str) -> IResult<&str, Vec<EmmetNode>, ErrorTree<&str>> {
    many0(preceded(char('+'), parse_emmet))(input)
}

fn parse_emmet(input: &str) -> IResult<&str, EmmetNode, ErrorTree<&str>> {
    let (input, mut node) = parse_emmet_node(input)?;

    let (input, children) = parse_children(input)?;
    node.children = children;

    let (input, siblings) = parse_siblings(input)?;
    node.siblings = siblings;

    Ok((input, node))
}

pub fn parse(input: String) -> Result<EmmetNode> {
    match parse_emmet(input.as_str()) {
        Ok((_, node)) => Ok(node),
        Err(e) => Err(anyhow!("Failed to parse Emmet expression: {}", e)),
    }
}
