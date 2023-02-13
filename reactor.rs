use std::process;
use std::io::Result;
use std::collections::HashMap;
use std::task::{Context, Waker};
use polling::{Event, Poller, Source};
use std::cell::RefCell;

thread_local! {
   pub static REACTOR: RefCell<Reactor> = RefCell::new(Reactor::new());
}
  
pub struct Reactor {
   readable: HashMap<usize, Vec<Waker>>,
   writable: HashMap<usize, Vec<Waker>>,
   poller: Poller,
}

impl Reactor {
    pub fn new() -> Self {
        Self {
           readable: HashMap::new(),
           writable: HashMap::new(),
           poller: Poller::new().unwrap_or_else(
              |err| {
                 eprintln!("\n\x1b[1;31m ERROR: something went wrong while creating poller\n inside Reactor struct:\n {}\x1b[0m", err);
                 process::exit(1);
              }
           ),
        }
        // self
    }
    
    fn get_interest(&self, key: usize) -> Event {
       let readable = self.read.contains(&key);
       let writable = self.writable.contains(&key);
        match (readable, writable) {
          (false, false) => Event::none(key),
          (true, false) => Event::readable(key),
          (false, true) => Event::writable(key),
          (true, true) => Event::all(key),
        }
    }

    pub fn add(&self, source: impl Source){
        let key = source.raw() as usize;
        self.poller.add(source, self.get_interest(key))
               .unwrap_or_else(
                 |err| {
                    eprintln!("\n\x1b[1;31m ERROR: something went wrong while\n  adding event source to poller inside Reactor struct:\n {}\x1b[0m", err);
                    process::exit(1);
                 });
    }

    pub fn remove(&mut self, source: impl Source){
        let key = source.raw() as usize;
        self.poller.delete(source).unwrap();
        self.readable.remove(&key);
        self.writable.remove(&key);
    }
    
    pub fn wait_on_readable(&mut self, source: impl Source, ctx: &mut Context){
        let key = source.raw() as usize;
        self.readable
              .entry(key)
               .or_default()
                .push(ctx.waker().clone());
        self.poller.modify(source, self.get_interest(key))
               .unwrap_or_else(
                 |err| {
                    eprintln!("\n\x1b[1;31m ERROR: something went wrong while\n  trying to modify poller in wait_on_readable() method:\n {}\x1b[0m", err);
                    process::exit(1);
                 });
    }

    pub fn wait_on_writable(&mut self, source: impl Source, ctx: &mut Context){
        let key = source.raw() as usize;
        self.writable
              .entry(key)
               .or_default()
                .push(ctx.waker().clone());
        self.poller.modify(source, self.get_interest(key))
               .unwrap_or_else(
                 |err| {
                    eprintln!("\n\x1b[1;31m ERROR: something went wrong while\n  trying to modify poller in wait_on_writable() method:\n {}\x1b[0m", err);
                    process::exit(1);
                 });
    }

    pub fn drain_wakers(&mut self, events: Vec<Event>) -> Vec<Waker> {
         let mut wakers = Vec::new();
         for event in events {
            if let Some(_, readers) = self.readable.remove_entry(&event.key) {
               for waker in readers {
                   wakers.push(waker);
               }
            } 
            if let Some(_, writers) = self.writable.remove_entry(&event.key) {
               for waker in writers {
                   wakers.push(waker);
               }
            } 
         }
         wakers
    }
  
    pub fn wait(&self, event: &mut Vec<Event>, timeout: Option<Duration>) -> Result<usize> {
        self.poller.wait(events, timeout);
    }
    
    pub fn wait_again(&self, events: &mut Vec<Event>, timeout: Option<Duration>) -> Result<usize> {
       REACTOR.with(|current| -> Result<()> {
            let mut events = Vec::new();
            {
               let reactor = current.borrow();
               reactor.wait(&mut events, None)?;
            }
            let wakers = {
               let mut reactor = current.borrow_mut();
               reactor.wakers(event)
            };
            for waker in wakers {
                 waker.wake();
            }
     
            Ok(())
         }
       )        
    }
     
    pub fn waiting_on_events(&self) -> bool {
        !self.readable.is_empty() || !self.writable.is_empty()
    }
}
// impl
