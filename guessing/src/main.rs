extern crate rand;

use std::sync::atomic::{AtomicI8, Ordering};
use std::{io, thread, time::Duration};
use std::cmp;
use rand::Rng;

fn delay(){
    thread::sleep(Duration::from_millis(1500));
}

fn adorn(ln: &String, tl: &String){
    println!("{}", ln);
    println!("{}", tl);    
    println!("{}", ln);
    delay();
}

fn start_game(ln: String, tl: String){
    adorn(&ln, &tl);
    println!("\t I have thought of a number between 1 and 10...");
    delay();
    println!("\t    Try to guess it if you can!");
    delay();
    let int_num: u32 = rand::thread_rng().gen_range(1, 11);
    static NUM_OF_ATTEMPTS: AtomicI8 = AtomicI8::new(1);

    loop {
        println!("\t\t Please enter your GUESS: ");
        delay();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(" Failed to get the input data...");
        let g = guess.clone();
        println!("====> You typed: {}", g);

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_)  => continue,
        };
        delay();

        match guess.cmp(&int_num) {
            cmp::Ordering::Less => {
                let this_trial = NUM_OF_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
                println!("\t Number of trials: {}. \n\t It is too small...", this_trial);
                delay();
            }
            cmp::Ordering::Greater => {
                let this_trial = NUM_OF_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
                println!("\t Number of trials: {} up to now. \n\t It is too big...", this_trial);
                delay();
            }
            cmp::Ordering::Equal => {
                let this_trial = NUM_OF_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
                println!("\t  BINGO! You have finally guess it! Total number of trials: {}.", this_trial);
                delay();
                break;
            }
        }
    }
    adorn(&ln, &tl);
}

fn main() {
    let line = String::from("\n=======================================================================================");
    let title = String::from("                  G U E S S I N G     G A M E");
    start_game(line, title);
}
