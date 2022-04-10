
pub trait Functor {
    fn fmap<T, U>(&self, func: T) -> U
        where T: Fn(&Self) -> U; 
}

impl Functor for i16 {
    fn fmap<T, U>(&self, func: T) -> U
        where T: Fn(&Self) -> U {
            func(&self)
    } 
}

impl Functor for i32 {
    fn fmap<T, U>(&self, func: T) -> U
        where T: Fn(&Self) -> U {
            func(&self)
    } 
}

impl Functor for i64 {
    fn fmap<T, U>(&self, func: T) -> U
        where T: Fn(&Self) -> U {
            func(&self)
    } 
}

impl Functor for u16 {
    fn fmap<T, U>(&self, func: T) -> U
        where T: Fn(&Self) -> U {
            func(&self)
    } 
}

impl Functor for u32 {
    fn fmap<T, U>(&self, func: T) -> U
        where T: Fn(&Self) -> U {
            func(&self)
    } 
}

impl Functor for u64 {
    fn fmap<T, U>(&self, func: T) -> U
        where T: Fn(&Self) -> U {
            func(&self)
    } 
}