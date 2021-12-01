use std::env;
use std::{thread, time::Duration};

fn main() {
    let movement = env::args().nth(1).expect("Please enter a command, that is: a number from 1 to 5!");
    let movement: u8 = movement.trim().parse().unwrap();
    
    motion(movement);
}

fn motion(cmd: u8) {
  let counter = 0; 
  loop{
      match cmd {
        1 => println!("\nRobot is moving forward!"),
        2 => println!("\nRobot is going backward!"),
        3 => println!("\nRobot is turning right!"),
        4 => println!("\nRobot is spinning left!"),
        _ => println!("\nRobot is stopped now..."),
      }
      delay();
      counter += 1
      if counter == 9 {
        println!("\nSorry, but now the robot is turning of...");
        delay_2();
        println!("\nGOODBYE, MASTER MARLON!");
        delay();
        break;
       }
   }
}

fn delay() {
   thread::sleep(Duration::from_secs(2));
}

fn delay_2() {
   thread::sleep(Duration::from_secs(5));
}
