use std::{thread, time::Duration};

enum Motion {
    Forward,
    Backward,
    Right,
    Left
}

fn move_robot(movement: Motion) {
   let moves = vec!["going ahead", "reversing now!", "turning right!", "moving left!"];
   println!("\n********************************************");  
  loop { 
     match movement {     
        Motion::Forward => println!("\n     Robot is {}\n", moves[0]),
        Motion::Backward => println!("\n     Robot is {}\n", moves[0]),
        Motion::Right => println!("\n     Robot is {}\n", moves[0]),
        Motion::Left => println!("\n     Robot is {}\n", moves[0]),
      }
      println!("********************************************\n");
      delay();
   }
}

fn main() {
    let f = Motion::Forward;
    let b = Motion::Backward;
    let r = Motion::Right;
    let l = Motion::Left;
    let mut counter = 0;
  
   loop {
      match counter {
        0 | 1 => move_robot(f),
        2 | 7 => move_robot(b),
        4 | 6  => move_robot(r),
        5 | 8  => move_robot(l),
        3 | 9  => move_robot(f),
             _ => {
            println!("\The robot is shutting down...\n");
           break;
         }
      }
      counter += 1;
   }
}

fn delay() {
  thread::sleep(Duration::from_secs(2));
}
