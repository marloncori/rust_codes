
pub mod no_extra_allocation {
    use std::mem;

    #[derive(Debug)]
    pub enum User {
        Reader {name: String},
        Writer {name: String},
        Admin {name: String},
    }

    pub fn promote(user: &mut User){
        use User::*;

        *user = match user {
            Reader { name } => Writer { name: mem::take(name) },
            Writer { name } => Admin { name: mem::take(name) },
             Admin { name: _ } => return,
        }
    }
}