use mem_take::no_extra_allocation::{User, promote};
use std::mem;

fn main() {
    let mut user: User = User::Reader{
        name: "Marlon".to_owned(),
    };

    println!("User = {user:?}");

    promote(&mut user);    
    println!("User = {user:?}");
    
    promote(&mut user);    
    println!("User = {user:?}");

    let mut name = String::from("Marlon");
    let number: u8 = name.as_bytes()[1];
    println!("ASCII number = {}", &number);
    println!("Letter = {}", number as char);
    println!("Name = {}", mem::take(&mut name));
    
}
