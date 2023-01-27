
pub mod container {
    use std::cmp;
    type Node = Option<u64>;

    // TimestampSaver <-- DynamicArray
    pub struct TimestampSaver {
        pub buffer: Box<[Node]>,
        pub capacity: usize,
        pub length: usize,
    }

    impl TimestampSaver {
        pub fn new (buffer: Box<[Node]>) -> Self {
            TimestampSaver {
                buffer: buffer.clone(),
                capacity: buffer.clone().len() + 10,
                length: buffer.len(),
            }
        }

        pub fn grow(&mut self, min_cap: usize){
            let old_cap = self.buffer.len();
            let mut new_cap = old_cap + (old_cap >> 1);
            
            new_cap = cmp::max(new_cap, min_cap);
            new_cap = cmp::min(new_cap, usize::max_value());

            let current = self.buffer.clone();
            self.capacity = new_cap;

            self.buffer = vec![None; new_cap].into_boxed_slice();
            self.buffer[..current.len()].clone_from_slice(&current);
        }

        pub fn at(&mut self, index: usize) -> Option<u64> {
            if self.length > index {
                self.buffer[index]
            } else {
                None
            }
        }
    }

    pub struct ListIterator {
        pub current: usize,
        data: Box<[Node]>,
    }

    impl Iterator for ListIterator {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            if self.current < self.data.len() {
                let item = self.data[self.current];
                self.current += 1;
                item
            } else  {
                None
            }
        }
    }

    impl DoubleEndedIterator for ListIterator {
        fn next_back(&mut self) -> Option<u64> {
            if self.current < self.data.len() {
                let item = self.data[self.current];
                if self.current == 0 {
                    self.current = self.data.len() - 1;
                } else {
                    self.current -= 1;
                }
                item
            } else {
                None
            }
        }
    }
}