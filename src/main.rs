use regex::Regex;
use std::env;
use std::process;
use colored::Colorize;
mod search;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Use: {} <dir num> <target dir1> <target dir2> ... <regex num> <target regular expression1> <target regular expression2> ... (-v)", args[0]);
        process::exit(1);
    }
    let tot_dir = &args[1].parse::<usize>().unwrap();
    let tot_reg = &args[tot_dir + 2].parse::<usize>().unwrap();
    let mut pattern = String::from(&args[tot_dir + 3]);
    for iter in (tot_dir + 4)..(tot_dir + tot_reg + 3) {
        let mut tmp = String::from("|");
        tmp += &args[iter];
        pattern += &tmp;
    }
    let regex = match Regex::new(pattern.as_str()) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("invalid expression '{}': {}", pattern.as_str(), err);
            process::exit(1);
        }
    };

    let mut match_yes = Vec::new();
    let mut match_no = Vec::new();

    for iter in 2..(tot_dir + 2) {
        let filepath = &args[iter];
        match search::find(filepath, &regex) {
            Ok((matches, cnt)) => {
                if cnt == 0 {
                    if args.len() > (3 + tot_dir + tot_reg) && (args[3 + tot_dir + tot_reg] == "-v" || args[3 + tot_dir + tot_reg] == "--verbose") {
                        for file in matches {
                            match_no.push(file);
                        }
                    }
                    //println!("didn't find any matches");
                } else {
                    println!("find following matches");
                    
                    if args.len() > (3 + tot_dir + tot_reg) && (args[3 + tot_dir + tot_reg] == "-v" || args[3 + tot_dir + tot_reg] == "--verbose") {
                        
                        let mut i = 0;
                        for file in matches {
                            if i < cnt {

                                //println!("{}", file.green().bold());
                                match_yes.push(file);
                            } else {
                                match_no.push(file);
                                //println!("{}", file.red());
                            }
                            i += 1;
                        }
                    } else {
                        let mut i = 0;
                        for file in matches {
                            match_yes.push(file);
                            //println!("{}", file.green().bold());
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
    if match_yes.is_empty() {
        println!("didn't find any matches");
        process::exit(0);
    }
    match_yes.sort();
    match_yes.dedup();
    for file in match_yes {
        println!("{}", file.green().bold());
    }
    if args.len() > (3 + tot_dir + tot_reg) && (args[3 + tot_dir + tot_reg] == "-v" || args[3 + tot_dir + tot_reg] == "--verbose") {
        match_no.sort();
        match_no.dedup();
        for file in match_no {
            println!("{}", file.red());
        }
    }
}