use std::fs::File;
use std::env;
use std::path::Path;
use log::debug;



fn help() {
    println!("Usage: ./integrity_checker <DIRECTORY>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: Option<&Path> = parse_arguments(&args);
    println!("The path enetered is : {}", path.unwrap().display());
}

fn parse_arguments(args: &Vec<String>) -> Option<&Path> {
    let directory: Option<&Path> = match args.len() {
        1 => {
            println!("I need some command to work! Use '!help' to get help");
            None
        },
        2 => {
            let path = Path::new(&args[1]);
            if path.exists() {
                println!("Looking for '{}'", args[1]);
                Some(path)
            } else {
                eprintln!("Path does not exist: '{}'", args[1]);
                None
            }
        },
        _ => {
            None
        }
    };

    directory
}
