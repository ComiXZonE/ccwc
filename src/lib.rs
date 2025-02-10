use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub command: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut file_path = String::from("");
        let mut command = String::from("");

        match args.len() {
            2 => { 
                file_path = args[1].clone(); 
            },
            3 => {
                command = args[1].clone();
                file_path = args[2].clone();
            },
            _ => return Err("Please specify file path")
        }

        Ok(Config { file_path, command })
    }
}

pub struct WordCount {
    pub contents: String,
}

impl WordCount {
    pub fn build(file_path: &str) -> Result<WordCount, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        Ok(WordCount { contents })
    }

    pub fn size_in_bytes(&self) -> usize {
        self.contents.len()
    }

    pub fn number_of_lines(&self) -> usize {
        self.contents.lines().count()
    }

    pub fn number_of_words(&self) -> usize {
        self.contents.split_whitespace().count()
    }

    pub fn number_of_chars(&self) -> usize {
        self.contents.chars().count()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let wc = WordCount::build(&config.file_path)?;

    match &config.command[..] {
        "-c" => print!("{} ", wc.size_in_bytes()),
        "-l" => print!("{} ", wc.number_of_lines()),
        "-w" => print!("{} ", wc.number_of_words()),
        "-m" => print!("{} ", wc.number_of_chars()),
        _ => {},
    }

    println!("{}", config.file_path);

    Ok(())
}