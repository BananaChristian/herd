use std::env;

pub enum CliArgs {
    ProjectName(String),
    Error(Option<String>),
    None,
}

pub fn print_help() {
    println!("Herd build system");
    println!("Usage: herd [options]");
    println!("Options: ");
    println!(" --help, -h             Show this help message");
    println!(" init <project_name>    Initialize the project");
}

pub fn handle_args() -> CliArgs {
    let args: Vec<String> = env::args().collect();
    let mut cli_arg = CliArgs::None;

    if args.len() < 2 {
        print_help();
        cli_arg = CliArgs::Error(Some("Invalid number of arguments".to_string()));
    }

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                i += 1;
                cli_arg = CliArgs::None;
            }
            "init" => {
                if i + 1 < args.len() {
                    cli_arg = CliArgs::ProjectName(args[i + 1].clone());
                    i += 2;
                } else {
                    cli_arg =
                        CliArgs::Error(Some("Missing project name for init command".to_string()));
                }
            }
            _ => {
                print_help();
                cli_arg = CliArgs::Error(Some(format!("Unknown flag '{}'", i)));
            }
        }
    }

    cli_arg
}
