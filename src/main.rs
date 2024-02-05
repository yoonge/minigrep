use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Not enough arguments.");
        process::exit(1);
    }
    println!("Search {} in file {}", args[1], args[2]);
    search(&args[1], &args[2]);
}


// struct Config {
//     query: String,
//     filename: String,
//     case_insensitive: bool,
// }

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let lines: Vec<&str> = contents.lines().collect();
    for line in lines {
        if line.contains(query) {
            println!("{}", line);
            results.push(line);
        }
    }
    results
}
