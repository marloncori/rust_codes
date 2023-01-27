use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};

fn main() {
    println!("\n  >>> Program is about to start...\n");
    delay();
        shared_state();
    delay();
    println!("\n   ===== End of Application =====\n");
}

fn delay(){
    thread::sleep(Duration::from_secs(1));
}

fn shared_state(){
    let value = Arc::new(Mutex::new(vec![]));
    let handles = (0..20).map(|i| {
        let nums = Arc::clone(&value);
        thread::spawn(move || {
            let mut my_vec= nums.lock().unwrap();
            (*my_vec).push(i.clone());
            println!(" ----> Value \'{}\' has been pushed into vector!", i);
            delay();
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }
    let line = "::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::".to_owned();
    println!("\t{}\n\tNumbers: {:?}\n\t{}\n",
        &line, *value.lock().unwrap(), line);
}