use std::{thread, time::Duration};

// 'a stands for with lifetime
struct Robot<'a> {
     name: &'static str;
     wheels: &'a i32
     dist: &'a f32
     cmd: &'a static str;
    }
       
  impl<'a> Robot<'a> {
     fn move(&self) -> &'static str { self.cmd }  
   }
           
fn main() {
    let person = "master".to_string();
    let n = "RM-347";
    let w = 4;
    let d = 78.912;
    let cmds: Vec<&str> = vec!["moving forward", "moving backward", "turning right", "turning left", "stopping motors", "slowing down"];
    let bot = Robot {name = &n, wheels = &w, dist = &d, cmd = cmds[3]};
    
    println!("\n-------- ROBOT ---- INFORMATION ------");
    wait();
    println!("     These are the info about your ROBOT:");
    wait_ms();
    println!("\n   Name: {}", bot.name);
    wait_ms();
    println!("     Wheels: {}", bot.wheels);
    wait_ms();
    println!("    Distance: {}", bot.dist);
    wait();
    println!("\nAnd this is the command sent by {}:", person);
    wait_ms();
       println!("         {}", bot.cmd);
    wait();
    println!("\n Goodbye, {}!\n", person); 
    wait_ms();
    println!("\n-------- END -- OF -- PROGRAM -----\n");
}
                    
fn wait() {
   thread::sleep(Duration::from_secs(2));
}
fn wait_ms() {
   thread::sleep(Duration::from_millis(600));
}
