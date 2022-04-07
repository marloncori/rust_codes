use std::error::Error;
use std::{fmt, io, thread, time::Duration};

type Result<String> = std::result::Result<String, DoubleError>;
type Double = u64;

#[derive(Debug)]
struct DoubleError;
impl Error for DoubleError {}
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t [WARNING] A Double error has happened!\n")
    }
}

fn delay() {
    thread::sleep(Duration::from_secs(1));
}

fn adorn(ln: String, tl: String) {
    println!("{}", ln);
    println!("{}", tl);
    println!("{}", ln);
}

fn get_data(ln: &str, tl: &str) -> Vec<Double> {
    let mut values: Vec<Double> = Vec::with_capacity(7);
    adorn(ln.to_string(), tl.to_string());
    delay();
    println!("\n :::: Welcome to the number analyser! :::: ");
    delay();
    println!("\n    Please enter seven numbers below.");
    delay();

    for x in 1..=7 {
        let mut n = String::new();
        let mut v = String::new();
        io::stdin().read_line(&mut n).expect(" An error happened while reading input data...");
        v = n.clone();
        println!("\t {}. You entered: {}", x, v);
        delay();
        let n: Double = n.trim().parse().unwrap();
        values.push(n);
    }
    values
}

fn number_analyser(val: Double) -> Result<String> {
    match val {
        0 | 1 | 2  => Err(DoubleError),
            2..=10 => Ok("\t\t It is a number between 2 and 10! \n".to_string()),
               _   => Ok("\t\t This number is neither a 0, nor 1, nor 2 \n\t   and it is not in the range of 2 to 10! \n".to_string()),
    }
}

fn vector_analyser(numbers: &[Double]) -> () {
    for num in numbers {
     match number_analyser(*num) {
                Ok(output)   => { 
                          println!("\t Ok! The number has passed the test: \n {}", output);
                          delay();
                        },
                Err(err_msg) => {
                          println!("\t Something went really wrong... \n {}", err_msg);
                          delay();
                        },
        }
    }
}

fn finish(ln: &str, tl: &str) {
    delay();    
    println!("\n\t >>>> Analysis has been successfully done! <<<<\n");
    delay();    
    adorn(ln.to_string(), tl.to_string());
}

fn main() {
    let line = "===============================================";
    let title = "       N U M B E R    A N A L Y S E R ";

    let num_vec = get_data(line, title);
    vector_analyser(num_vec.as_slice());

    finish(line, title);
}