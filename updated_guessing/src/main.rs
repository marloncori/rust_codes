use updated_guessing::game;
use std::{io, cmp::Ordering};

fn main() {
    let mut right = false;
    let mut attempt = game::Counter::new();
    let age: i32 = 35; 

    println!("\n Try to guess how old I am...");

    while right != true {
        print!(" --> ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(" Problem reading user input...");
        let guess: i32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if guess != 0 {
            let hunch = game::Guess::new(guess);

            match hunch.value.cmp(&age) {
                Ordering::Less => {
                    println!("\t Too small, not yet...");
                    attempt.next();
                    continue;
                },
                Ordering::Greater =>{
                    println!("\t Nice, but it was too big...");
                    attempt.next();
                    continue;
                },
                 Ordering::Equal => {
                    println!("\t Great! That is right!");
                    attempt.next();
                    println!("\t\tNumber of trials = {}", attempt.attempts);
                    right = true;
                },
        
            }
        }
    }
}