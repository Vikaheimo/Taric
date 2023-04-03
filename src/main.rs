use std::{env, io};
mod lookup;
use std::str::FromStr;

const HELP_MESSAGE: &'static str = "You summoned me summoner!
Taric is a commnd-line application to lookup League of Legends stats.

Running taric from the command line with two extra parameters,
will do a lookup for a summoner. Example:

taric SUMMONER_NAME EUNE

would lookup a player named SUMMONER_NAME from the EUNE server.
Server can be in lowecase or uppercase. Summoner names are case sensitive.
If your summoner name contains a space, then leave it out. Example:

taric SUMMONERWITHSPACES EUW

could lookup SUMMONER WITH SPACES on the EUW server.
If you don't specify any arguments the program will be run in interactive mode,
which uses the same logic to get your input.";

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => interactive_mode(),
        2 => print_help_message(),
        _ => single_mode(&args[1..=2]),
    }
    println!("{:?}", args);
}

fn single_mode(values: &[String]) {
    println!("{:?}", values)
}

fn interactive_mode() {
    println!("Starting interactive mode!");
    loop {
        let mut input: String = String::from("");
        println!("Please enter a summoner name and a server seperated with a to search:");
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("You entered an empty value!");
                continue;
            }
        }
        println!();

        let splitted_input: Vec<String> = input
            .split(" ")
            .map(|s| {
                let mut new = s.to_string();
                trim_newline(&mut new);
                new
            })
            .collect();
        
        if splitted_input.len() < 2 {
            println!("Please enter two arguments in the form of: SummonerName Server");
            continue;
        }
        single_mode(&splitted_input[0..=1])
    }
}

fn print_help_message() {
    println!("{}", HELP_MESSAGE)
}

fn trim_newline(string: &mut String) {
    if string.ends_with('\n') {
        string.pop();
        if string.ends_with('\r') {
            string.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trim_newline;

    #[test]
    fn test_newline_trimming() {
        let mut start_string = "asdsad\r\n".to_owned();
        let equals = "asdsad".to_owned();
        trim_newline(&mut start_string);
        assert_eq!(start_string, equals)
    }

    #[test]
    fn doesnt_touch_no_newlines() {
        let mut start_string = "mogus".to_owned();
        let equals = "mogus".to_owned();
        trim_newline(&mut start_string);
        assert_eq!(start_string, equals)
    }
}
