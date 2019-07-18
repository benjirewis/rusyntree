use std::collections::HashMap;

use regex::Regex;

use crate::{
    element::{Element, ElementType},
    error::{Error, Result},
};

lazy_static! {
    static ref ALL_SPACES: Regex = Regex::new(r"\[\s*\]").unwrap();
    static ref BRACK_MATCH: Regex = Regex::new(r"\[([^\[\]]|(?R))*\]").unwrap();
    static ref BRACK_SPACES: Regex = Regex::new(r" \[").unwrap();
    static ref BRACKS_SPACES: Regex = Regex::new(r"\] \[").unwrap();
    static ref MULT_SPACES: Regex = Regex::new(r"\s+").unwrap();
}

/// Parses a phrase into leafs and nodes and stores the result in a
/// vector of elements.
#[derive(TypedBuilder)]
pub struct Parser {
    /// The data to be parsed
    data: String,

    /// ID for the next element
    #[builder(default = 1)]
    id: usize,

    /// Current level in the diagram
    #[builder(default)]
    level: usize,

    /// Counts of node types
    #[builder(default_code = "HashMap::new()")]
    node_types: HashMap<String, usize>,

    /// Current position in sentence
    #[builder(default)]
    pos: usize,

    /// The vector to hold the resulting parse
    #[builder(default_code = "Vec::new()")]
    result: Vec<Element>,
}

fn is_valid(data: &str) -> bool {
    if data.len() < 1 {
        return false;
    }

    let regex_check = match (ALL_SPACES.captures(data), BRACK_MATCH.captures(data)) {
        (None, None) => false,
        (None, Some(_)) => true,
        (Some(_), None) => false,
        (Some(_), Some(_)) => false,
    };

    regex_check
}

impl Parser {
    pub fn new(mut data: String) -> Result<Self> {
        // Validate data
        if !is_valid(&data) {
            return Err(Error::Parse("invalid data provided to parser".to_string()));
        }

        // Sanitize data for processing
        data = data.replace("\t", "");
        data = MULT_SPACES.replace_all(&data, " ").to_string();
        data = BRACKS_SPACES.replace_all(&data, "][").to_string();
        data = BRACK_SPACES.replace_all(&data, "[").to_string();

        Ok(Parser::builder().data(data).build())
    }

    fn auto_subscript(&mut self) {
        let mut temp_count: HashMap<String, usize> = HashMap::new();

        self.result = self
            .result
            .iter()
            .map(|element| {
                let mut new_element = element.clone();

                if element.element_type == ElementType::Branch {
                    let count = match self.node_types.get(&element.content) {
                        Some(cnt) => *cnt,
                        None => 1,
                    };

                    if count > 1 {
                        let subscript = match temp_count.get(&element.content) {
                            Some(cnt) => *cnt + 1,
                            None => 1,
                        };

                        temp_count.insert(element.content.clone(), subscript);

                        new_element
                            .content
                            .push_str(&format!("_{}", subscript.to_string()));
                    }
                }

                new_element
            })
            .collect();
    }

    fn count_node(&mut self, name: String) {
        let name = name.trim();
        let count = match self.node_types.get(name) {
            Some(cnt) => *cnt,
            None => 1,
        };
        self.node_types.insert(name.to_string(), count);
    }

    fn next_token(&mut self) -> String {
        let chars: Vec<char> = self.data.chars().collect();
        let mut token = String::new();
        let mut got_token = false;

        if self.pos + 1 >= chars.len() {
            return "".to_string();
        }

        let mut i = 0;
        let mut escape = false;
        while (self.pos + i < chars.len()) && !got_token {
            let ch = chars[self.pos + i];
            match ch {
                '[' => {
                    if escape {
                        token.push(ch);
                        escape = false;
                    } else {
                        if i > 0 {
                            got_token = true;
                        } else {
                            token.push(ch);
                        }
                    }
                }
                ']' => {
                    if escape {
                        token.push(ch);
                        escape = false;
                    } else {
                        if i == 0 {
                            token.push(ch);
                        }
                        got_token = true;
                    }
                }
                '\\' => escape = true,
                '\n' | '\r' => got_token = false,
                _ => {
                    token.push(ch);
                    if escape {
                        escape = false;
                    }
                }
            }

            i += 1;
        }

        if i > 1 {
            self.pos += i - 1;
        } else {
            self.pos += 1;
        }

        token.trim().to_string()
    }

    /// Parse `self.data` into `self.result` recursively.
    pub fn parse(&mut self) {
        self.parse_recurse(0);
    }

    fn parse_recurse(&mut self, parent: usize) {
        let mut token = self.next_token();
        let mut parts: Vec<String> = Vec::new();

        while !token.is_empty() && token != "]".to_string() {
            let mut token_chars: Vec<char> = token.chars().collect();

            match token_chars[0] {
                '[' => {
                    token_chars = token_chars.split_off(1);
                    let space_at = token_chars.iter().position(|c| *c == ' ').unwrap_or(0);
                    let new_parent;

                    if space_at != 0 {
                        let mut part: String = token_chars[0..space_at].iter().collect();
                        part = part.replace("<>", " ");
                        parts.push(part);

                        part = token_chars[space_at..].iter().collect();
                        part = part.replace("<>", " ");
                        parts.push(part);

                        let element = Element::new(self.id, parts[0].clone(), parent, self.level);
                        new_parent = element.id;
                        self.id += 1;
                        self.result.push(element);
                        self.count_node(parts[0].clone());

                        let element =
                            Element::new(self.id, parts[1].clone(), self.id - 1, self.level + 1);
                        self.id += 1;
                        self.result.push(element);
                    } else {
                        let joined: String = token_chars.iter().collect();

                        let element = Element::new(self.id, joined.clone(), parent, self.level);
                        new_parent = element.id;
                        self.id += 1;
                        self.result.push(element);
                        self.count_node(joined);
                    }

                    self.level += 1;
                    self.parse_recurse(new_parent);
                }
                _ => {
                    let element = Element::new(self.id, token.to_string(), parent, self.level);
                    self.id += 1;
                    self.result.push(element);
                    self.count_node(token.to_string());
                }
            }

            token = self.next_token();
        }

        self.level -= 1;
    }

    /// Return current stored result.
    pub fn result(&self) -> Vec<Element> {
        self.result.clone()
    }
}
