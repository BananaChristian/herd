use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use crate::{config::Config, layout::ProjectLayout};

fn create_build_toml(project_name: &String) -> Result<(), std::io::Error> {
    let config = Config::new(project_name.clone());
    let toml_string = toml::to_string_pretty(&config).expect("Failed to serialize to TOML");
    let path = format!("{}/{}", project_name, "build.toml");
    fs::write(path, toml_string)?;

    Ok(())
}

fn create_example(path: &String) -> Result<(), std::io::Error> {
    //For now only C
    let full_path = format!("{}/{}", path, "main.c");
    let file = File::create(full_path)?;
    let mut writer = BufWriter::new(file);
    let content = r#"
#include <stdio.h>
    
int main(){
    printf("Hello World!\n");
    return 0;
}
    "#;

    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    Ok(())
}

fn create_project(layout: &ProjectLayout) -> Result<(), std::io::Error> {
    let root = layout.get_root_dir();
    //Create the project root
    fs::create_dir(&root)?;

    //Create build.toml
    create_build_toml(&root)?;

    //Create the sub dirs
    let src_dir = format!("{}/{}", root, layout.get_src_dir());
    let include_dir = format!("{}/{}", root, layout.get_include_dir());
    let build_dir = format!("{}/{}", root, layout.get_build_dir());

    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(include_dir)?;
    fs::create_dir_all(build_dir)?;

    //Create the main.c stub
    create_example(&src_dir)?;

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
