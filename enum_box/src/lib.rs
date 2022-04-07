use std::fmt;
use std::boxed::Box;

trait Showable {
    fn show(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " --> The Point has been successfully created!")        
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        print!("  ==> The point has been properly deleted!");
    }
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Point{
            x, y, 
        }
    }
}

impl Showable for Point {
    fn show(&self) -> String {
        let ln = String::from("\n\t--------------------------------------\n");
        let msg = format!("{}\tNew Point \n\t  x: {}, \n\t  y: {} {}", &ln, self.x, self.y, ln);
        msg.to_string()
    }
}

#[derive(Debug)]
pub enum List {
    Empty,
    Cons(Point, Box<List>),
}