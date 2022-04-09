use publish::{Summary, TimeStamp, Article};
use publish::{Content, Tweet, Comment};
use std::boxed::Box;
use std::{thread, time::Duration};

struct Website {
    elems: Vec<Box<dyn Summary>>,
    comms: Vec<Box<dyn TimeStamp>>,
}

fn notify_all<T, U>(item1: &Box<T>, item2: &Box<U>) 
   where T: Summary + ?Sized,
         U: TimeStamp + ?Sized
{
   let ln = String::from("\n\t====================================\n");
   print!("{} {} {} {}", &ln, 
         item1.summarize(), 
         item2.time_stamp(), ln
    );
}

fn write() -> impl Summary {
   let tweet2 = Tweet::new(
    String::from("Bogdan Milikovic"),
    Content::new(
        String::from(" Hey Rusteers! Thank you for the highly \n\t esteemed support! I am glad I can \n\t teach. May Rut live long and safely!
        \n\t Let us go on improving the community!")
    ),
    true
);
   tweet2 
}

fn delay() {
   thread::sleep(Duration::from_millis(2000));
}
fn main() {
    let news = Article::new(
            String::from("Marlon Ribeiro"),
            String::from(" LEARN RUST WITH VIDEOS"),
        Content::new(
            String::from(" There is a very good YouTube Channel 
            \n\t where you can learn a lot about Rust. 
            \n\t Clear and concise lesson! The channel is  
            \n\t called: \'Let\'s Get Rusty\'. Nice!
             \n\t Its creator is Bogdan Milikovic 
            \n\t a half-American and half-Ukranian citizen.
            \n\t He publishes contents there every week.")
        )
    );
    let tweet = Tweet::new(
        String::from("Beniamin Couto"),
        Content::new(
            String::from(" Hey guys! There is a YouTube Channel I want you to 
            \n\tknow about: it is LET\'S GET RUSTY.
            \n\t If you are seirous about learning Rust, \n\tthat is the right place!")
        ),
        true
    );
    let comm1 = Comment::new(
        Content::new(
            String::from(" \t\tBogdan is the best teacher indeed. \n\t\tThank for all the great videos!")
        ),
        String::from(" ==> Friday, 08 April 2022 - 21:55:01")
    );

    let comm2 = Comment::new(
        Content::new(
            String::from(" \t\t I have watched other videos, \n\t\t but I think he is the best one \n\t to teach us about Rust programming.")
        ),
        String::from(" ==> Friday, 07 April 2022 - 14:23:47")
    );


    let site = Website {
        elems: vec![
            Box::new(news), 
              Box::new(tweet)
        ],
        comms: vec![
            Box::new(comm1), 
              Box::new(comm2)
        ],
    };

    for elem in site.elems.iter() {
        for comm in site.comms.iter() {
            notify_all(elem, comm);
            delay();
        }
    }
    println!("\n{}", write().summarize());
}
