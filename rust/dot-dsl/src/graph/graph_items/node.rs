use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    name: String,
    attributes: HashMap<String, String>
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            name: name.to_string(),
            attributes: HashMap::new()
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        attrs.iter().for_each(|&(key, value)| {
            self.attributes.insert(key.to_string(), value.to_string());
        });

        self
    }

    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}