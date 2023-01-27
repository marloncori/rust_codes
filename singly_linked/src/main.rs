use std::{thread, time::Duration};
use singly_linked::linked_list;

fn main() {
    let mut counter: u8 = 0;
    let mut logger = linked_list::TransactionLog::new_empty();

    let sentences: Vec<String> = vec![
        String::from(" --> Robot is moving forward."),
        String::from(" ---> Camera is turned on. "),
        String::from(" ----> Battery is low, charge me ASAP!"),
        String::from(" -----> The obstacle is at 20 cm."),
        String::from(" -------> The robot is leaning to the left by 30 degrees."),
        String::from(" --------> Speed boosted up from 180 to 230!"),
        String::from(" ---------> Green led is on and buzzer is beeping!")
    ];

    start();

    for words in sentences {
        println!(" >>>> Sentence \'{}\' appended to logger!", &words);
        logger.append(words);
        delay();
    }

   println!("\n");
   while logger.get_size() != 0 {
       logger.pop();
       counter += 1;

       println!("\t ::::: {}. A value has been popped from the Logger!", &counter);
       delay();
   } 

   finish();
}

fn delay(){
    thread::sleep(Duration::from_millis(500));
}

fn start(){
    println!("\n\t   >>>> PROGRAM IS ABOUT TO START <<<< \n");
    delay();
}

fn finish(){
    let line = "-------------------------------------------------\n";
    println!("\n\n {} >>>> PROGRAM HAS ENDED. ==> GOODBYE! <<<< \n", line);
    delay();
}