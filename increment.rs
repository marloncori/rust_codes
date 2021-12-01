use std::borrow::Cow;
use std::{thread, time::Duration};

#[derive(Clone)]
struct Thing;

impl Thing {
    fn do_something(&self) -> (usize, Thing) {
        (1, Thing)
    }
}

fn increment(n: &Thing) -> usize {
    let mut n = Cow::Borrowed(n);
    let mut x = 0;

    {
        let (q, m) = n.do_something();
        n = Cow::Owned(m);
        x = x + q;
    }

    x
}

fn main() {
 loop{
   let mut counter = increment(&Thing);
   println!("----> Counter x = {}\n", counter);
   delay();
  }
}

fn delay() {
  thread::sleep(Duration::from_secs(3));
}
