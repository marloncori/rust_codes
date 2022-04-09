
pub trait Summary {
    fn summarize(&self) -> String {
        let ln = String::from("\n\t---------------------------------\n");
        format!("{} \t Read more... {}", &ln, ln)
    }
}

pub trait TimeStamp {
    fn time_stamp(&self) -> String;
}

#[derive(Debug)]
pub struct Content {
    pub text: String,
}

impl Content {
    pub fn new(text: String) -> Self {
        Content {
            text,
        }
    }
}

#[derive(Debug)]
pub struct Article {
  pub author: String,
  pub headline: String,
  pub content: Content,
}

impl Article {
   pub fn new(author: String, headline: String,
               content: Content) -> Self {
        Article { 
            author, headline, content, 
        }
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("\n\t :::: {} ::::\n\n\t ==> {:?} \n\t\t by {}", self.headline, self.content, self.author)
     }
}

pub struct Tweet {
  pub username: String, 
  pub content: Content,
  pub publish: bool,
}

impl Tweet {
   pub fn new(username: String, content: Content,
            publish: bool) -> Self {
     Tweet { 
        username, content, publish, 
     }
   }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
       format!("\n\t[ Twitter ] \n\t\t user: {} \n\t\'{:?}\'", self.username, self.content)
    }
}

pub struct Comment {
  pub sentence: Content,
  pub creation: String,
}

impl Comment {
   pub fn new(sentence: Content, creation: String) -> Self {
     Comment { 
       sentence,
       creation,  
     }
   }
}

impl TimeStamp for Comment {
    fn time_stamp(&self) -> String {
        format!("\n\t\t Creation date: {} \n\t {:?}", self.creation, self.sentence)
    }
}