use deref::Sensor;
use std::{thread, time::Duration};

fn wait() {
    thread::sleep(Duration::from_secs(2));
}

fn main() {
    let distances = vec![2.3, 45.6, 0.79, 56.7, 1.88, 103.45];
    let poses = vec![98, 123, 17, 54, 39, 100, 72];
    let ranges = vec![88.75, 32.10, 54.89, 134.57, 34.56, 100.23, 97.32, 64.23, 77.88, 21.09, 1.45];
    
    let mut hc_sr04 = Sensor::new(distances);
    let sg_90 = Sensor::new(poses);
    let mpu_6050 = Sensor::new(Vec::<f32>::new());

    let line = String::from("\n\t------------------------------------------\n");
    let line2 = String::from("\n\t------------------------------------------\n");

    match hc_sr04.has_data() {
        Ok (_) => {
                let mut ct = 0;
                println!("{}\t The length of data stored \n\tin dist reading vector: {} {}", &line, hc_sr04.readings.len(), &line2);
                for r in hc_sr04.readings.iter() {
                        println!("\t ==> Reading {}: {} cm", ct.clone()+1, r);
                        ct += 1;
                        wait();
                };
            },
        Err(error) => println!("{}", error),
    }
    wait();

    match sg_90.has_data() {
        Ok(_) => {
                let mut ct2 = 0;
                println!("{}\t The length of data stored \n\tin SG-90 pose vector: {} {}", line.clone(), sg_90.readings.len(), line2.clone());
                for r in hc_sr04.readings.iter() {
                    println!("\t ==> Position {}: {} degrees", ct2.clone()+1, r);
                    ct2 += 1;
                    wait();
                    
                };
              },
         Err(error)  => println!("{}", error),
    }
    wait();

    *hc_sr04 = ranges;
    match mpu_6050.has_data() {
        Ok (_) => {
                let mut ct3 = 0;
                println!("\t The length of data stored \n\tin dist reading vector: {}", mpu_6050.readings.len());
                 for r in mpu_6050.readings.iter() {
                   println!("\t ==> Reading {}: {} x y z", ct3.clone()+1, r);
                   ct3 += 1;
                   wait();
                 };
            },
        Err(error) => println!("{} {} {}", &line, error, &line2),
    }

    wait();
    match hc_sr04.has_data() {
        Ok (_) => {
                let mut ct = 0;
                println!("{}\t The length of data stored \n\tin dist reading vector: {} {}", line, hc_sr04.readings.len(), line2);
                for r in hc_sr04.readings.iter() {
                    println!("\t ==> Reading {}: {} cm", ct.clone()+1, r);
                    ct += 1;
                    wait();
                };
            },
        Err(error) => println!("{}", error),
    }
}
