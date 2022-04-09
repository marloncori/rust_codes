use std::{env, thread, time::Duration};
use minigrep::{Minigrep, Config, InvalidArgsError};
use std::error::Error;
use std::boxed::Box;
use std::process;

fn delay(time: u64){
    thread::sleep(Duration::from_millis(time));
}

fn parse_config<'a>(args: &'a Vec<String>) -> Result<Config, InvalidArgsError> {
    if args.len() < 3 {
        return Err(InvalidArgsError);
    }

    let query = &args[1];
    let filename = &args[2];

    let query_filename = Config::new(query, filename);
    Ok(query_filename)
}

fn start_search<'a>(args: &'a Vec<String>) -> Result<(), Box<dyn Error>> {
    let searcher = Minigrep::new(
        parse_config(&args).unwrap_or_else(|err|{
            println!("\n ==> Problem parsing arguments! \n Cause => {}", err);
            process::exit(1);
        })
    );
    delay(1000);
    searcher.init()
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if let Err(e) = start_search(&args) {
        println!("\n Application error: \n\t{}", e);
        process::exit(1);
    }
}
