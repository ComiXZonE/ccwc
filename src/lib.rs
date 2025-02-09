use std::error::Error;

pub struct Config {
    pub file_path: String,
    pub command: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 3 {
            return Err("too many arguments");
        }

        let file_path = args[1].clone();
        let command = args[2].clone();

        Ok(Config { file_path, command })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}