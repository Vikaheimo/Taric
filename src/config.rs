use std::{path::{PathBuf, Path}, fs::{File, self}, fmt::{Error, Display}, io::{self, BufRead, Write}};


#[derive(Debug, Default)]
pub struct Config {
    token: String,
}

impl Config {
    pub fn from_file(filepath: &PathBuf) -> Result<Self, Box<dyn std::error::Error>>{
        for line in read_lines(filepath)? {
            let mut line_string = match line {
                Err(_) => continue,
                Ok(value) => value,
            };
            crate::trim_newline(&mut line_string);
            if line_string.starts_with("token=") && line_string.len() > 6 {
                return Ok(Config { token: line_string[6..].to_string() });
            }
        }
        
        return Err("Config file doesn't contain token information!".into());
    }

    pub fn setup_config(filepath: &PathBuf) {
        let mut token: String = String::from("");
        println!("Please enter your riot API token!");
        match io::stdin().read_line(&mut token) {
            Ok(_) => (),
            Err(_) => {
                println!("You entered an empty value!");
                return;
            }
        }

        let mut file = match fs::File::create(filepath) {
            Ok(file) => file,
            Err(error1) => match fs::File::open(filepath) {
                Ok(file) => file,
                Err(error2) => {
                    println!("Error creating/editing files");
                    println!("{}", error1);
                    println!("{}", error2);
                    return;
                }
            }
        };

        match file.write_all(format!("token={}", token).as_bytes()) {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        }
    }
}

impl Display for Config{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}