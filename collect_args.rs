use std::env;
use std::{thread, time::Duration};

fn main(){
   let args: Vec<String> = env::args().collect();
     
     for i in 0..args.len() {
         println!("==");
         println!(" {}", args[i]);
         println!("==");
         wait();
     }
}

fn wait() {
   thread::sleep(Duration::from_millis(700));     
}
