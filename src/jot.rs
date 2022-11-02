use std::{io::{self, Read}, collections::HashSet, fmt::Display, path::{Path, PathBuf}, fs::File};

pub struct Jot {
    pub title: String,
    pub variant: JotVariant,
    pub tags: HashSet<String>,
    pub content: String,
    pub archived: bool
}

pub enum JotVariant {
    Note,
    Task,
    Journal
}

pub trait JotObj {
    fn new(title: String, tags: HashSet<String>, content: String) -> Jot;
    fn parse(path: PathBuf) -> Jot;
    fn update() -> Result<(), io::Error>;
}

impl Default for Jot {
    fn default() -> Self {
        Self { title: Default::default(), variant: JotVariant::Note, tags: Default::default(), content: Default::default(), archived: Default::default() }
    }
}

impl JotObj for Jot {
    fn new(title: String, tags: HashSet<String>, content: String) -> Jot {
        Jot { title: title, tags: tags, content: content, variant: JotVariant::Note, archived: false }
    }

    fn parse(path_str: PathBuf) -> Jot {
        let binding = path_str.clone();
        let filename = binding.file_name();
        let path = File::open(path_str);

        match path {
            Ok(mut f) => {
                let mut content = String::default();
                let _ = f.read_to_string(&mut content);

                let lines = content.split("\r\n").collect::<Vec<&str>>();

                let title =  if let Some(f) = filename { f } else { todo!() };

                let variant = match lines[1].to_lowercase().as_ref() {
                    "note" => JotVariant::Note,
                    "task" => JotVariant::Task,
                    "journal" => JotVariant::Journal,
                    _  => JotVariant::Note
                };

                let mut tags = HashSet::new();

                for str in lines[1].split(" ") {
                    tags.insert(str.to_owned());
                }

                let content = lines[2..].join("\n");

                return Jot::new(title.to_str().unwrap().to_string(), tags, content);
            },
            Err(e) => println!("{}", e.to_string())
        }
        
        Jot::default()
    }

    fn update() -> Result<(), io::Error> {
        todo!()
    }
}

impl Display for Jot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tags_concatenated = self.tags.iter().map(|x| x.clone()).collect::<Vec<String>>().join(" ");

        f.write_fmt(format_args!("{}-{}\n{}", self.title, tags_concatenated, self.content))
    }
}

pub struct Jots {
    all: Vec<Jot>
}

pub trait JotCollection {
    fn search(tags: Vec<String>) -> Vec<Jot>;
}