use std::env;

use crate::scaffold::scaffold;
use colored::*;
mod layout;
mod scaffold;

fn print_help() {
    println!("Herd build system");
    println!("Usage: herd [options]");
    println!("Options: \n");
    println!(" -h, --help                Display this help message");
    println!(" init <project_name>        Creates a new project with a specified name");
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("init") => match args.get(2) {
            Some(name) => match scaffold(name.to_string()) {
                Ok(_) => cli_message(format!("Created project {}", name)),
                Err(err) => cli_error(format!("Failed to create project {} due to {}", name, err)),
            },
            None => {
                print_help();
                cli_error(format!("Error: herd init requires a project name"))
            }
        },
        Some("--help") | Some("-h")=> print_help(),
        None => {
            print_help();
            cli_error(format!("No command was provided"));
        }
        _ => {
            match args.get(1) {
                Some(n) => {
                    print_help();
                    cli_error(format!("Error: unknown command {} was provided", n));
                }
                None => {
                    print_help();
                    cli_error("No command was provided".to_string());
                }
            };
        }
    }
    Ok(())
}

fn cli_error(message: String) {
    eprintln!("{}", message.red());
    std::process::exit(1);
}

fn cli_message(message: String) {
    println!("{}", message.green());
}
