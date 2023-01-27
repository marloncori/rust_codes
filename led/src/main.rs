use std::process::Command;
use std::io;

fn main(){
  let mut input = String::new();
  println!("\n ---------------------------------------\n\x1b[1;32m Enter a command below:\x1b[0m");
 
 io::stdin().read_line(&mut input)
         .expect("\x1b[1;31mAn unexpected error has happened...\x1b[0m");
  
  let mut cmd = Command::new("NucLedCli.exe");
  cmd.arg(input);
    println!("\n ---------------------------\n\x1b[1;33m --> Command executed.\x1b[0m");
}