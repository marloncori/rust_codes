
fn main() {
    print_numbers();
}

fn print_numbers() {
    for n in 1..=100 {
        if n % 3 == 0 {
            println!("\t Fizz!");
        } else if n % 5 == 0 {
            println!("\t Buzz!");
        } else if n % 3 == 0 && n % 5 == 0 {
            println!("\t FizzBuzz!");
        } else {
            println!(" ==> {}", n);
        }
    }
}