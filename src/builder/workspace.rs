use std::fs;

use crate::{config::Config, layout::ProjectLayout};

///The workspace manager
#[derive(Debug)]
pub struct Workspace {
    ///A collection of the source files
    sources: Vec<String>,
    ///Provides the layout of the project
    layout: ProjectLayout,
    ///Will provide the rules the the workspace manager must follow
    config: Config,
}

impl Workspace {
    pub fn new(config: &Config) -> Self {
        Workspace {
            sources: Vec::new(),
            layout: ProjectLayout::new(config.project.get_name()),
            config: config.clone(),
        }
    }

    pub fn scan_dir(&mut self, dir: &str) -> Result<(), std::io::Error> {
        //For now I will only collect c files
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                self.scan_dir(dir)?;
            } else if path.extension() == Some(".c".as_ref()) {
                self.sources.push(path.to_string_lossy().into_owned());
            }
        }
        Ok(())
    }

    pub fn get_layout(&self) -> ProjectLayout {
        self.layout.clone()
    }
}
