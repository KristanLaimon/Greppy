use std::{env, io, io::BufRead, process};

fn main()  {
    let args: Vec<String> = env::args().collect();
    let config: Config  = Config::new(&args);

    let stdin = io::stdin();
    let mut i:i32 = 0;
    for line in stdin.lock().lines().into_iter(){
        match line {
            Ok(line) => {
                let is_match : bool = if config.case_insensitive {
                    line.to_lowercase().contains(&config.text_to_search)
                }else {
                    line.contains(&config.text_to_search)
                };

                if is_match {
                    i += 1;
                    println!("{}.  {}", i, line)
                }
            }
            Err(e) => {
                println!("Error reading a line from stdin. Error: {:?}", e);
            }
        }
    }
       
}

mod Colors {
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const RESET: &'static str = "\x1b[0m";
}

struct Config {
     text_to_search: String,
     case_insensitive: bool
}

impl Config {
    fn new(args:&[String]) -> Config {
        let mut case_insensitive = false;
        let mut pattern: Option<String> = None;

        for arg in args.iter().skip(1) {
            if arg == "-i" {
                case_insensitive = true;
            }else if pattern.is_none(){
                pattern = if case_insensitive { Some(arg.to_lowercase()) } else { Some(arg.clone()) };
            }
        }
        
        let text_to_search = pattern.unwrap_or_else(|| {
            eprintln!("Usage: greppy <pattern>");
            process::exit(1);
        });

        Config { text_to_search, case_insensitive }
    }
}
