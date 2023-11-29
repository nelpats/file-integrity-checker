use std::fs::File;
use std::{env, thread};
use std::path::Path;
use std::time::Duration;
use sha2::{Sha256, Digest};
use tokio::{fs, io};
use std::collections::HashMap;

fn help() {
    println!("Usage: ./integrity_checker <DIRECTORY>");
}

async fn get_file_content(file_path: &Path) -> io::Result<Vec<u8>> {
    if file_path.is_file() {
        Ok(fs::read(file_path)?)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Provided path is not a file",
        ))
    }
}

async fn on_file_modified() {
    println!("A file got tampered with !!");
}


async fn integrity_routine(files: &mut HashMap<String, String>, path : &Path) {
    let duration = Duration::from_secs(10);
    loop {
        tokio::time::sleep(duration).await;
        println!("Performing integrity check...");

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    let file_content = get_file_content(file_path).await;
                    let mut hasher = Sha256::new();
                    let digest = hasher.update(file_content).finalize().to_string(); // TODO: fix the problem with the hash library
                    println!("Hash for the file {}: {}", entry, digest);
                    files.entry(file_path).or_insert(digest.clone());

                    if files.get(file_path).expect("hash digest") != digest {
                        on_file_modified().await;
                    }



                }
            }
        } else {
            eprintln!("Error reading directory");
        }

    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path: Option<&Path> = parse_arguments(&args);
    let mut files: HashMap<String, String> = HashMap::new();

    match path {
        Some(path) => {
            integrity_routine(&mut files, path).await;
        },

        None => {}

    };
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
