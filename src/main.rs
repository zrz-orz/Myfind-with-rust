use regex::Regex;
use std::env;
use std::process;
use colored::Colorize;
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
        Ok((matches, cnt)) => {
            if matches.is_empty() {
                println!("didn't find any matches");
            } else {
                println!("find following matches");
                
                if args.len() > 3 && (args[3] == "-v" || args[3] == "--verbose") {
                    
                    let mut i = 0;
                    for file in &matches {
                        if i < cnt {
                            println!("{}", file.green().bold());
                        } else {
                            println!("{}", file.red());
                        }
                        i += 1;
                    }
                } else {
                    let mut i = 0;
                    for file in &matches {
                        println!("{}", file.green().bold());
                        i += 1;
                        if i >= cnt {
                            break;
                        }
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Error:{}", error);
            process::exit(1);
        }
    }
}