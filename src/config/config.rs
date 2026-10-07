use serde::Serialize;

use crate::config::project::Project;

#[derive(Serialize)]
pub struct Config {
    project: Project,
}

impl Config {
    pub fn new(project_name: String) -> Self {
        Config {
            project: Project::new(project_name),
        }
    }
}
