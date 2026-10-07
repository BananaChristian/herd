///This dictates the project layout
#[derive(Clone, Debug)]
pub struct ProjectLayout {
    ///The root of the project
    root: String,
    ///Where the implementation files sit
    src: String,
    ///Where the header files sit
    include: String,
    ///Where the build output is places
    build: String,
}

impl ProjectLayout {
    pub fn new(root: &String) -> Self {
        ProjectLayout {
            root: root.clone(),
            src: format!("{}/src", root),
            include: format!("{}/include", root),
            build: format!("{}/build", root),
        }
    }

    pub fn get_root_dir(&self) -> String {
        self.root.clone()
    }

    pub fn get_src_dir(&self) -> String {
        self.src.clone()
    }

    pub fn get_include_dir(&self) -> String {
        self.include.clone()
    }

    pub fn get_build_dir(&self) -> String {
        self.build.clone()
    }
}
