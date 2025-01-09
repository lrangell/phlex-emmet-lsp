use bon::bon;

#[derive(Debug, Clone, PartialEq)]
pub struct EmmetNode {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub children: Vec<EmmetNode>,
    pub siblings: Vec<EmmetNode>,
    pub multiplier: usize,
    pub text: Option<String>,
}

type Children = Vec<EmmetNode>;
type Siblings = Vec<EmmetNode>;

#[bon]
impl EmmetNode {
    #[builder]
    pub fn new(
        tag: &str,
        id: Option<String>,
        classes: Option<Vec<String>>,
        children: Option<Children>,
        siblings: Option<Siblings>,
    ) -> Self {
        EmmetNode {
            tag: tag.to_owned(),
            id,
            classes: classes.unwrap_or_default(),
            children: children.unwrap_or_default(),
            siblings: siblings.unwrap_or_default(),
            multiplier: 1,
            text: None,
        }
    }
    pub fn from_tag(tag: &str) -> EmmetNode {
        EmmetNode {
            tag: tag.to_owned(),
            id: None,
            classes: vec![],
            children: vec![],
            siblings: vec![],
            multiplier: 1,
            text: None,
        }
    }

    #[builder]
    pub fn add(
        &mut self,
        child: Option<EmmetNode>,
        sibling: Option<EmmetNode>,
        child_str: Option<&str>,
        sibling_str: Option<&str>,
        id: Option<&str>,
        text: Option<&str>,
        class: Option<&str>,
    ) -> Self {
        if let Some(c) = child {
            self.children.push(c)
        }
        if let Some(s) = sibling {
            self.siblings.push(s)
        }

        if let Some(c_str) = child_str {
            self.children.push(from_tag(c_str))
        }
        if let Some(s_str) = sibling_str {
            self.siblings.push(from_tag(s_str))
        }
        if let Some(id_str) = id {
            self.id = Some(id_str.to_string())
        }
        if let Some(text) = text {
            self.text = Some(text.to_string())
        }
        if let Some(class) = class {
            self.classes.push(class.to_string());
        }

        self.to_owned()
    }

    pub fn add_child(&mut self, child: EmmetNode) -> Self {
        self.add().child(child).call()
    }

    pub fn add_sibling(&mut self, sibling: EmmetNode) -> Self {
        self.add().sibling(sibling).call()
    }
}

pub fn from_tag(tag: &str) -> EmmetNode {
    EmmetNode::builder().tag(tag).build()
}
