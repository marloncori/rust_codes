
pub mod stringify {
    use std::io;
    
    pub fn adorn(line: &str, title: &str) {
        println!("\n{}", &line);
        println!("\n{}", title);
        println!("\n{}", line);
    }

    pub fn get_username() -> io::Result<String> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        println!("\n\t You entered the name --> \'{}\'", &buffer);
        Ok(buffer)
    }
}