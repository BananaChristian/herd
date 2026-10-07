use serde::{Deserialize, Serialize};

use crate::config::project::Project;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub project: Project,
}

impl Config {
    pub fn new(project_name: String) -> Self {
        Config {
            project: Project::new(project_name),
        }
    }
}
