use regex::Regex;
use std::env;
use std::process;
mod search;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Use: {} <target dir> <target regular expression>", args[0]);
        process::exit(1);
    }

    let pattern = &args[2];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("invalid expression '{}': {}", args[2], err);
            process::exit(1);
        }
    };

    match search::find(&args[1], &regex) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("didn't find any matches");
            } else {
                println!("find following matches");
                for file in matches {
                    println!("{}", file);
                }
            }
        }
        Err(error) => {
            eprintln!("Error:{}", error);
            process::exit(1);
        }
    }
}