use cacher::Cacher;
use std::{thread, time::Duration};
use std::collections::HashMap;
use std::cmp;

fn delay() {
    thread::sleep(Duration::from_secs(2));
}

fn gen_workout (intense: u32, number: u32) {
    let mut closure1 = Cacher::new(|num: u32| -> u32 {
        println!("\n\t Generating workout...");
        let result = num * 4 * 2 + 1200 / 3 - 7;
        delay();
        result
    });

    let mut closure2 = Cacher::new(|num: u32| -> u32 {
        println!("\n\t Calculating second part...");
        let result = num * 4 * 2 + 900 / 3 - 7;
        delay();
        result
    });
    let number2 = &number * 3 / 2;
    let line = String::from("\n\t---------------------------------------\n");
    let line2 = String::from("\n\t========================================\n");
    let threshold: u32 = 25;
    
    match intense.cmp(&threshold) {
        cmp::Ordering::Less  => {
             println!("{}\t [GYM INFO] Do up to \n\t {} push-ups today! {}",
                &line, closure1.value(number-2), line
             );
             delay();
             println!("{}\t [GYM INFO] Please do next \n\t {} sit-ups before \n\t taking a good break.{}",
                &line2, closure2.value(number2-4), line2
             );
        },
        cmp::Ordering::Equal => {
            println!("{}\t [GYM INFO] Do up to \n\t {} push-ups today! {}",
                &line, closure1.value(number+25), line
            );
            delay();
            println!("{}\t [GYM INFO] Please do next \n\t {} sit-ups before \n\t taking a good break. {}",
                &line2, closure2.value(number2+10), line2
             );
        },
        cmp::Ordering::Greater => {
            println!("{}\t [GYM INFO] Do up to \n\t {} push-ups today! {}",
                &line, closure1.value(number/2), line
            );
            delay();
            println!("{}\t [GYM INFO] Please do next \n\t {} sit-ups before \n\t taking a good break. {}",
                &line2, closure2.value(number2/3), line2
             );
        }
    };
}

fn rest_day() {
    println!(" You can take a good break today!");
}

fn main() {
    let mut values: Vec<u32> = vec![24, 25, 26, 27];
    let mut params: HashMap<u32, u32> = HashMap::new();

    for i in 24..28 {
        params.insert(i, i.clone()+5);
    }

    for v in values.iter_mut() {
        match params.get(v) {
            Some(arg) => {
                delay();
                gen_workout(*v, *arg);
                delay();
            },
            None => rest_day(),
        }
    }
}
