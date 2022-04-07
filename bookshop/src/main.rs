use bookshop::{Book, Comment, Customer, Summary};
use std::boxed::Box;
use std::cell::RefCell;

struct Bookstore {
    inside: Vec<Box<dyn Summary>>,
}

impl Bookstore {
    fn open(&self) {
        for elem in self.inside.iter() {
            println!("\n{}", elem.summarize());
        }
    }
}

fn main() {
    let my_book = Book::new(String::from("\'Learn C++ Programming\'"),
                         String::from("Mark Peterson"));

    let mut my_comment = Comment::new(RefCell::new(
        String::from("\n Thank you so much for the offered discount. I am sure this book will be a great source of concrete knowledge. I am looking forward to starting coding in C++!")), 
                               String::from("Monday, 04 April 2022"));

    my_comment.update(RefCell::new(String::from("\n Thank you so much for the offered discount!\n I am sure this book will be\n a great source of concrete knowledge. I am looking\n forward to starting coding in C++!"))).unwrap();                    
 
    let new_customer = Customer::new(String::from("Marlon Ribeiro"),
                              my_comment); 
    let book2 = Book::new(String::from("Rust Programming By Examples"), String::from("Bogdan Milikovic"));

    let mut comment2 = Comment::new(RefCell::new(String::from(" Nothing to comment here.")),
                               String::from("Tuesday, 05 April 2022"));

    comment2.update(RefCell::new(String::from("\n This is one of the best books\n I have ever read about Rust \n so I strongly recommend it!"))).unwrap();

    let customer2 = Customer::new(String::from("Beniamin Couto-Szalay"),
                                comment2);

    let bookstore = Bookstore {
        inside: vec![
            Box::new(my_book), 
                Box::new(new_customer),
                  Box::new(book2),
                     Box::new(customer2)
        ],
    };

    bookstore.open();
}
