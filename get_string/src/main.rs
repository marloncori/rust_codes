use get_string::stringify;

fn main() {
    let ln = "================================";
    let tl = "  I O : : R E S U L T < String >";
    let m  = "     Please enter your name!";

    stringify::adorn(ln, m);
    
    match stringify::get_username() {
        Ok(name) => println!("\n\t I hope you are doing great, master {}!", name),
            _ => println!("\n\t Keep up the good work, master!"),
    };
    
    stringify::adorn(ln, tl);
}
