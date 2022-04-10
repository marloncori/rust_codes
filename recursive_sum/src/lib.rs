
pub mod operation {

    #[derive(Debug)]
    pub enum NumberList {
        Empty,
        Cons(i32, Box<NumberList>),
    }

    pub fn sum_list(list: &NumberList) -> i32 {
        match *list {
            NumberList::Empty => 0,
            NumberList::Cons(elem, ref rest) =>
                        elem + sum_list(rest),
        }
    }

    pub fn is_empty(list: &NumberList) -> bool {
        if let NumberList::Empty = *list {
            true
        } else {
            false
        }
    }
}