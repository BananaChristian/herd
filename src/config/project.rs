use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Project {
    name: String,
}

impl Project {
    pub fn new(name: String) -> Self {
        Project { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
