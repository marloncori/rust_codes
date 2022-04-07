 use std::cell::RefCell;
 use std::error::Error;
 use std::{fmt, thread, time::Duration};

 pub trait Summary {
     fn summarize(&self) -> String;
 }

 pub struct Book {
    pub title: String,
    pub author: String,
 }

 impl Book {
     pub fn new(title: String, author: String) -> Self {
         Book {
             title, author,
         }
     }
 }

 impl Summary for Book {
     fn summarize(&self) -> String {
         thread::sleep(Duration::from_secs(2));
         let line = String::from("\n------------------\n");
         format!("{} TITLE:\n {}\n\n AUTHOR:\n {} {}",
            &line, self.title, self.author, line)
     }
  }

 #[derive(Debug)]
 pub struct InvalidContentError;
 impl Error for InvalidContentError {}
 
 impl fmt::Display for InvalidContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " There is no valid content in this comment!")
    }
 }

 #[derive(Debug, Clone)]
 pub struct Comment {
     pub content: RefCell<String>,
     pub date: String,  
 }

 impl Comment {
     pub fn new(content: RefCell<String>, date: String) -> Self {
         Comment {
            content, date,
         }
     }

     pub fn update(&mut self, new_comment: RefCell<String>) -> Result<RefCell<String>, InvalidContentError> {
         let content = new_comment.clone();
         Ok(content)
     }
 }

 impl fmt::Display for Comment {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "")
     }
 }

 impl Summary for Comment {
     fn summarize(&self) -> String {
         thread::sleep(Duration::from_secs(2));
         let line = String::from("\n================================\n");
         format!("{} \tCONTENT:\n \'{}\'\n\n PUBLISHED:\n {}\n {}", &line, self.content.borrow_mut(), self.date, line)
     }

 }

 pub struct Customer {
     pub name: String,
     pub comment: Comment, 
  }

  impl Customer {
     pub fn new(name: String, comment: Comment) -> Self {
         Customer {
             name, comment,
         }
     }
  }

 impl Summary for Customer {
     fn summarize(&self) -> String {
         thread::sleep(Duration::from_secs(2));
         let line = String::from("\n:::::::::::::::::::::::::::::::\n");
         format!("{} NAME:\n {}\n\n COMMENT:\n\t {}\n {}", &line, self.name, self.comment.summarize(), line)
     }
  }
