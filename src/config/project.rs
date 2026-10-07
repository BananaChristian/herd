use serde::Serialize;

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    name: String,
}

impl Project {
    pub fn new(name: String) -> Self {
        Project { name }
    }
}
