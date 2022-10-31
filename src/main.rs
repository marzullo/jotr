use std::{
    default,
    fs::{write, File},
    io::{Read, Write, self, BufRead}, path::{PathBuf, Path},
};

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: String::default(),
        }
    }
}

#[derive(Debug, ValueEnum, Clone)]
enum CommandType {
    New,
    Edit,
    List,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(value_enum)]
    command: CommandType,
}

fn main() {
    println!("{:x?}", Path::new("C:\\").exists());

    let config_file = get_config_file().expect("Failed to get configuration file");

    match parse_config(config_file) {
        Ok(config) => {
            if config.directory == String::default() {
                println!("Please configure a directory:");

                let mut directory = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut directory).unwrap();

                if Path::new(directory.strip_suffix("\r\n").unwrap()).exists() {
                    println!("Path configured!");
                } else {
                    println!("Path does not exist.");

                    return;
                }
            }
        }
        Err(err) => println!("{}", err),
    }

    let args = Args::parse();

    match args {
        New => new(),
        List => list(),
        _ => print!("Command not supported"),
    }
}

fn get_config_file() -> std::io::Result<File> {
    let mut config_file = File::open("config.toml");

    if config_file.is_err() {
        config_file = File::create("config.toml");

        let config = Config::default();

        let toml = toml::to_string(&config).unwrap();

        let mut file = config_file.unwrap();
        file.write_all(toml.as_bytes())?;

        return Ok(file);
    }

    config_file
}

fn parse_config(mut file: File) -> Result<Config, toml::de::Error> {
    let mut contents = String::new();
    let bytes = file.read_to_string(&mut contents);

    let parsed_config: Config = toml::from_str(&contents)?;

    Ok(parsed_config)
}

fn new() {
    println!("New")
}

fn list() {
    println!("List")
}
