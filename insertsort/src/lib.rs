
pub mod algorithm {
    pub struct Sortable {
         pub list: Vec<i16>,
    }

    impl Sortable {
        pub fn new(list: Vec<i16>) -> Self {
            Sortable {
                list,
            }
        }
        
        pub fn insert_sort(&mut self) -> Vec<i16> {
           for this_elem in 1..self.list.len() {
                let accumulator = self.list[this_elem];
                let mut prev_elem = this_elem - 1;
                while prev_elem >= 0 && accumulator < self.list[prev_elem] {
                    self.list[prev_elem+1] = self.list[prev_elem];
                    prev_elem -= 1;
                }
                self.list[prev_elem+1] = accumulator;
           }
           self.list.clone()
        }

    }
}

