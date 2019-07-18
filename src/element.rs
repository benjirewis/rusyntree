use regex::Regex;

/// A struct that represents a basic tree element, either node or leaf.
#[derive(Clone, Debug, TypedBuilder)]
pub struct Element {
    pub id: usize,

    pub content: String,

    pub parent: usize,

    pub level: usize,

    pub triangle: bool,

    #[builder(default)]
    pub element_type: ElementType,

    #[builder(default)]
    pub width: usize,

    #[builder(default)]
    pub indent: usize,
}

impl Element {
    pub(crate) fn new(id: usize, content: String, parent: usize, level: usize) -> Self {
        let re = Regex::new(r"\A.\^\z").unwrap();
        let mut new_content = content.clone();
        let mut triangle = false;

        if re.captures(&content).is_some() {
            new_content.replace("^", "");
            triangle = true;
        }

        Element::builder()
            .id(id)
            .content(new_content)
            .parent(parent)
            .level(level)
            .triangle(triangle)
            .build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ElementType {
    Branch = 1,
    Leaf = 2,
    Undefined,
}

impl Default for ElementType {
    fn default() -> Self {
        ElementType::Leaf
    }
}
