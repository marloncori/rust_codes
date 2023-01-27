use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    println!("\n >>> App is about to start... >>> \n");

    delay(); 
    channels();
    delay();
    
    println!("\n:::: Program has ended. ::::");
}

fn channels(){
    let n: i32 = 10;
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    let handles = (0..n).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            let _ = _tx.send(i).unwrap();
        })       
    });

    // close all threads    
    for h in handles{
        h.join().unwrap();
    }
    // receive N times
    let numbers: Vec<i32> = (0..n).map(|_|
            rx.recv().unwrap()
        ).collect();
        delay();

        println!("\n\t Thread has been spawned! \n\t Value is: {:?}", numbers);
        delay();    
}

fn delay(){
    thread::sleep(Duration::from_millis(2000));
}