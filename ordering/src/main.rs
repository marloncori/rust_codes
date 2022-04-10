use std::cmp::Ordering;
use std::env;


fn main() {
    let num: u32 = 42;
    let msg = test_value(num.clone());
    println!("\n ==> The program chose number {} and received the \n  following message: \n\t {}", num, msg);

    println!("\n Now it is your turn to choose a number, please.");
    let arg_num = env::args().nth(1).expect(" Please ener an integer as the argument for the second function!");
    let int_num = arg_num.trim().parse::<u32>().expect("Problem parsing the number");

    println!("\n\t Another message has arrived after you typed: {}!", &int_num);
    let msg2 = compare_value(int_num);
    println!("\n\t\t--> {:?}", msg2);
}

fn test_value(number: u32) -> String {
    let setpoint: u32 = 70;
        println!("\n Setpoint is {} and you entered: {}...", &setpoint, &number);
    let reply = match number.cmp(&setpoint) {
        Ordering::Less => " Number is smaller than 70!",
        Ordering::Equal => " Number is equal to setpoint!",
        Ordering::Greater => " Number is greater than setpoint!",
    };
    reply.to_string()
}

fn compare_value(val: u32) -> String {
    let text = match val {
      val if val<42 => " Value is less than 42!",
      val if val>42 => " Value is greater than 42!",
         42 =>   " Wow! Value equals 42!",
      1..=10 =>   " Value is in range from 1 to ten!"      ,
         _ =>   " You have type an unsigned value!",
    };
    String::from(text)
}