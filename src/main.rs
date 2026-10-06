mod driver;

fn main() -> Result<(), std::io::Error> {
    let arg = driver::handle_args();

    let mut project_name = String::new();

    match arg {
        driver::CliArgs::ProjectName(name) => project_name = name,
        driver::CliArgs::Error(msg) => match msg {
            Some(text) => cli_error(text),
            None => cli_error("Unexpected error occured".to_string()),
        },
        driver::CliArgs::None => (),
    }

    println!("PROJECT NAME: {}", project_name);
    Ok(())
}

fn cli_error(message: String) {
    eprintln!("{}", message);
    std::process::exit(1);
}
