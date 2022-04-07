use std::io;
use std::{thread, time::Duration};

fn main() {

    let mut m = String::new();
    let mut n = String::new();
    let line: String = "-------------------------------------------------".to_string();
    let tl: String = "   G R E A T E S T   C O M M O N   D I V I S O R".to_string();

    println!("{}", line);
    println!("{}", tl);
    println!("{}", line);
    delay();

    println!("\n\t Enter two numbers for us to get their GCD.");
    delay();
    io::stdin().read_line(&mut n).expect("An unexpected error happened!");
    let num1 = n.clone();
    print!("\t\tValue 1: {}", num1); 
    delay();
    let n: u64 = n.trim().parse().unwrap();

    io::stdin().read_line(&mut m).expect("An unexpected error happened!");
    let num2 = m.clone();
    print!("\t\tValue 2: {}", num2); 
    delay();
    let m: u64 = m.trim().parse().unwrap();

    let gcd = greatest_common_divisor(n, m);
    delay();

    println!("\n\t Their greatest common divisor is: {}\n", gcd);
    delay();
    println!("{}", line);
    println!("{}", tl);
    println!("{}", line);
}

fn greatest_common_divisor(mut value1: u64,
                           mut value2: u64) -> u64 {
   //  it uses Euclid's algorithm
    assert!(value1 != 0 && value2 != 0);
    while value2 != 0 {
        if value2 < value1 {
            let accumulator = value2;
                value2 = value1;
                value1 = accumulator;
        }
        value2 = value2 % value1;
    }
    value1
}

fn delay() {
    thread::sleep(Duration::from_secs(1));
}

#[test]
fn test_gcd(){
    assert_eq!(greatest_common_divisor(14, 16), 2);

    assert_eq!(greatest_common_divisor(2*3*5*11*17, 
                                3*7*11*13*19), 3*111);
}