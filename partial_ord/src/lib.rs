
pub mod num_analysis {

    pub fn find_min<T: PartialOrd>(x: T, y: T) -> Option<T> {
           match x {
            x if x<y => Some(x),
               _     => Some(y),
           }
    }

    pub fn find_max<T: PartialOrd>(x: T, y: T) -> Option<T> {
        match x {
            x if x>y => Some(x),
               _     => Some(y),
           }
   }

}