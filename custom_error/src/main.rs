use std::error::Error;
use std::boxed::Box;
use std::{fmt, io , thread, time::Duration};

#[derive(Debug)]
struct IntError;
impl Error for IntError {}
impl fmt::Display for IntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t WARNING: An integer error has happened!")
    }
}

#[derive(Debug)]
struct ZeroError;
impl Error for ZeroError {}
impl fmt::Display for ZeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t WARNING: A zero exception was caught!")
    }
}

fn number_analyser(num: Option<i32>) -> Result<String, Box< dyn Error>> {
    match num {
        Some(x) => match x {
                   0    => Err(Box::new(ZeroError)),
                -1 | 1  => Err(Box::new(IntError)),
                10..=19 => Ok("\t ==> We have analysed number in range 10 to 19.".to_string()),
                   _    => Ok("\t --> The analysed number is neither zero nor 1 or -1.".to_string()),
        },
        None   => Ok("The variable is empty! An analysis cannot be done!".to_string()),
    }
}

fn delay(){
   thread::sleep(Duration::from_secs(1));
}

fn get_input(line: &str, title: &str) -> Option<i32> {
    println!("{}", line);
    println!("{}", title);
    println!("{}", line);
    println!("\n  Please enter a number below to be analysed.");
    delay();
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("An error has ocurred while getting input data.");
    let val = num.clone();
    println!("\n\t You typed: {}", val);
    let num: i32 = num.trim().parse().expect("Something went wrong while parsing String to integer.");
    Some(num)
}

fn show_result(input: Result<String, Box< dyn Error>>, line: &str, title: &str) {
    println!("\n\t This is the analysis result: \n\t {:?}", input);
    println!("{}", line);
    println!("{}", title);
    println!("{}", line);
}

fn main() {
    let ln: &str = "====================================================";
    let tl: &str = "  C U S T O M   E R R O R   A P P L I C A T I O N";

    let output = match get_input(&ln, &tl) {
        Some(value)  => Some(value),
        None         => None,
    };

    let analysis = match number_analyser(output) {
        Ok(number)   => Ok(number),
        Err(msg)     => Ok(msg.to_string()),
    };

    show_result(analysis, &ln, &tl);
}
