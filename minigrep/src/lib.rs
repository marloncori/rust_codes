use std::{fmt, thread, time::Duration};
use std::error::Error;
use std::boxed::Box;
use std::{process, fs};

#[derive(Debug)]
pub struct InvalidArgsError;
impl Error for InvalidArgsError {}
impl fmt::Display for InvalidArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " WARNING: Please provide two arguments: \n\t arg1: query \n\t arg2: file name \n Without that Minigrep cannot search anything! \n")
    }
}

#[derive(Debug)]
pub struct FileNotFoundError;
impl Error for FileNotFoundError {}
impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " Even though I tried it hard, unfortunately requested file \n has not been found in root folder. \n\t Please make sure whether it exists or \n\t you have typed the correct name. \n\t I\'m sorry, but without that Minigrep cannot help you.\n")
    }
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(query: &'a str, file: &'a str) -> Self {
        Config {
            query, file, 
        }
    }
}

pub struct Minigrep<'a> {
    pub input: Config<'a>,
}

impl<'a> Minigrep<'a> {
    pub fn new(input: Config<'a>) -> Self {
        Minigrep {
            input,
        }
    }

    fn wait(&self, time: u64){
        thread::sleep(Duration::from_millis(time));
    }

    pub fn search<'b>(query: &str, contents: &'b str) -> Result<Vec<&'b str>, FileNotFoundError> {
        let mut results = Vec::new();
    
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        if results.is_empty(){
            return Err(FileNotFoundError);
        }
        Ok(results)
    }
    
    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        let line = String::from("\t-------------------------------------------------------");
     println!("\n{}", &line);
     self.wait(500);
     println!("\n\t   M I N I G R E P    S E A R C H E R");
     self.wait(500);
     println!("{}", &line);
     self.wait(500);
     println!("\t\t\t version: 1.0.1\n\t\t\t author: Marlon Couto\n\t\t         <millenium_robotics@hotmail.com>");
     self.wait(500);
     println!("\n\t Welcome to Minigrep Searcher!" );
     self.wait(500);
     println!("\n\n\t User has passed some arguments!");
     self.wait(1000);
     println!("\n\t\t ==> Query: \'{}\', searching for it", &self.input.query);
     self.wait(200);
     println!("\n\t\t  in file: \'{}\' . Please wait...", &self.input.file);
     let contents = fs::read_to_string(self.input.file).unwrap_or_else(|err|{
        println!("\n [ WARNING ]: Problem reading the file! \n Cause: \n\t{}", err);
        process::exit(1);
     });

     self.wait(2000);
     let result: Vec<&str> = Minigrep::<'a>::search(self.input.query, &contents).unwrap_or_else(|err|{
        println!("\n --> Problem in searching query in the file! \n Cause: \n\t{}", err);
        process::exit(1);
     });
     
     println!("\n\t\t ==> One match has been found: \n\t{:?}", result);
     self.wait(500);
     println!("\n\t\t Thank you for using Minigrep. Goodbye!");
     self.wait(500);
     println!("\n{}", &line);
     self.wait(500);
     println!("\n\t   M I N I G R E P    S E A R C H E R");
     self.wait(300);
     println!("{}", &line);
     Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents =  " \
        Rust: \n safe, fast, productive. \n Pick all three!";

        assert_eq!(vec![" safe, fast, productive. "], mock_search(query, contents));
    }
}