use crate::builder::Workspace;

pub struct Builder {
    workspace: Workspace,
}

impl Builder {
    pub fn new(workspace: Workspace) -> Self {
        Builder { workspace }
    }

    pub fn build(&self) {
        println!("BUILDING");
    }
}
