use crossterm::event::{read, Event};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self, stdin, stdout, Read, Write};
use structopt::StructOpt;

fn get_path(args: Command) -> io::Result<String> {
    match args.path {
        Some(p) => Ok(p),
        None => {
            print!("Enter the file path to work on: ");
            stdout().flush()?;
            let mut path = String::new();
            stdin().read_line(&mut path)?;
            Ok(path.trim().to_owned())
        }
    }
}

fn get_content(path: String) -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn display(content: String) -> Result<(), Box<dyn Error>> {
    for c in content.chars() {
        if let Event::Key(_) = read()? {
            print!("{}", c);
            stdout().flush()?;
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "work-on",
    about = "A command used to work on a file by typing random things on the keyboard."
)]
struct Command {
    #[structopt(help = "The path to the file to work on")]
    path: Option<String>,
}

fn main() -> Result<(), String> {
    let args = Command::from_args();

    let path = get_path(args).map_err(|e| format!("{}", e))?;
    let content = get_content(path).map_err(|e| format!("{}", e))?;
    display(content).map_err(|e| format!("{}", e))?;

    Ok(())
}
