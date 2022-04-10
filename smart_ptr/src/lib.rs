
#[derive(Debug)]
pub enum List<T> {
    Empty,
    Cons(T, Box<List<T>>)
}

#[derive(Debug)]
pub enum Stack<T> {
    Empty,
    Cons(T, Box<Stack<T>>)
}


#[derive(Debug)]
pub enum Pile<T> {
    Empty,
    Cons(T, Box<Pile<T>>)
}