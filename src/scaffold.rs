use std::fs;

use crate::layout::ProjectLayout;

fn create_project(layout: &ProjectLayout) -> Result<(), std::io::Error> {
    let root = layout.get_root_dir();
    //Create the project root
    fs::create_dir(&root)?;
    //Should create build.toml here

    //Create the sub dirs
    let src_dir = format!("{}/{}", root, layout.get_src_dir());
    let include_dir = format!("{}/{}", root, layout.get_include_dir());
    let build_dir = format!("{}/{}", root, layout.get_build_dir());

    fs::create_dir_all(src_dir)?;
    fs::create_dir_all(include_dir)?;
    fs::create_dir_all(build_dir)?;

    //Should create the example main.c or c++ file

    Ok(())
}

pub fn scaffold(project_name: String) -> Result<(), std::io::Error> {
    let layout = ProjectLayout::new(
        project_name,
        "src".to_string(),
        "include".to_string(),
        "build".to_string(),
    );
    create_project(&layout)
}
