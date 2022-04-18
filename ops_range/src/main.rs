use std::{ops, thread, time::Duration};

fn main() {
    let range1 = 20..=40;
    let range2 = 5..19;

    inclusive(range1);
    sleep(3000);
    exclusive(range2);
    sleep(500);
}

fn sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}

fn inclusive(seq: ops::RangeInclusive<i16>) {
    for (index, num) in seq.enumerate() {
        println!("\t ==> Number {} at index {}.", num, index);
        sleep(2000);
    }
}

fn exclusive(seq: ops::Range<i16>) {
    for (index, num) in seq.enumerate() {
        println!("\t ==> At index {} we find number {}.", index, num);
        sleep(1000);
    }
}