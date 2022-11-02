pub mod jot;

use std::{
    default,
    fs::{write, File, self},
    io::{Read, Write, self, BufRead}, path::{PathBuf, Path}, error::Error,
};

use clap::{Parser, ValueEnum};
use jot::{Jot, JotObj};
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
    Search,
    Archive
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(value_enum)]
    command: CommandType,

    #[arg(value_name = "[tags]")]
    tags: Option<String>
}

fn main() {
    let config_file: File;
    let config_parsed: Config;
    
    match get_config_file() {
        Ok(f) => config_file = f,
        Err(_) => {
            println!("Could not retrieve config.");
            return;
        }
    }

    match parse_config(config_file) {
        Ok(config) => {
            if config.directory == String::default() {
                println!("Please configure a directory:");

                let mut directory = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut directory).unwrap();

                directory = directory.strip_suffix("\r\n").unwrap().to_string();

                if Path::new(&directory).exists() {
                    println!("Path configured!");

                    fs::create_dir(directory.clone() + "\\archive").unwrap();

                    write_config_file(Config { directory: directory }).unwrap();
                } else {
                    println!("Path does not exist.");

                    return;
                }
            }

            config_parsed = config;
        }
        Err(err) => { 
            println!("{}", err);
            return;
        },
    }

    let args: Args = Args::parse();

    match args.command {
        CommandType::New => todo!(),
        CommandType::Edit => todo!(),
        CommandType::List => list(config_parsed),
        CommandType::Search => search(config_parsed, args.tags.unwrap().split(",").map(|x| x.to_owned()).collect()),
        CommandType::Archive => todo!(),
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

fn write_config_file(config: Config) -> Result<(), Box<dyn Error>> {
    let mut config_file = File::options().write(true).open("config.toml")?;

    config_file.write_all(toml::to_string(&config)?.as_bytes())?;

    Ok(())
}


fn parse_config(mut file: File) -> Result<Config, toml::de::Error> {
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    let parsed_config: Config = toml::from_str(&contents)?;

    Ok(parsed_config)
}

fn new() {
    println!("New")
}

fn list(config: Config) {
    let path = Path::new(&config.directory);

    for entry_result in path.read_dir().unwrap() {
        if let Ok(entry) = entry_result {
            if entry.file_type().unwrap().is_dir() {
                continue;
            }

            let jot = Jot::parse(entry.path());

            println!("{}", jot);
        }
    }
}

fn search(config: Config, tags: Vec<String>) {
    let path = Path::new(&config.directory);

    for entry_result in path.read_dir().unwrap() {
        if let Ok(entry) = entry_result {
            let jot = Jot::parse(entry.path());

            for tag in tags.iter() {
                if jot.tags.contains(tag) {
                    println!("{}", jot);
                }
            }
        }
    }
}
