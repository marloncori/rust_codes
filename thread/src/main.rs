use std::thread;
use std::time::Duration;

fn main() {
    let x = 10;

    println!("\n >>> App is about to start... >>> \n");
    delay();
    
    let handle = thread::spawn(move || {
        println!("\n\t Thread has been spawned! \n\t Value is: {}", x);
    });
    handle.join().unwrap();
    
    delay();
    println!("\n:::: Program has ended. ::::");
}

fn delay(){
    thread::sleep(Duration::from_millis(2000));
}