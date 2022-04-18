

pub struct UnsortedList {
    pub vector: Vec<i16>,
}

impl UnsortedList {
    pub fn new(vector: Vec<i16>) -> Self {
        UnsortedList {
            vector,
        }
    }

    fn partition(&mut self, begin: usize, end: usize) -> usize {
        let pivot = self.vector[end];
        let mut i = begin - 1;
        for j in begin..end {
            if self.vector[j] <= pivot {
                i += 1;
                let temp = self.vector[i];
                self.vector[i] = self.vector[j];
                self.vector[j] = temp;
            }
        }
        let temp = self.vector[i+1];
        self.vector[i+1] = self.vector[end];
        self.vector[end] = temp;
        i+1
    }

    pub fn quick_sort(&mut self, begin: usize, end: usize) {
        if begin < end {
            let partition_index = self.partition(begin, end);
            self.quick_sort(begin, partition_index-1);
            self.quick_sort(partition_index+1, end);
        }
    }

    pub fn show_sorted(&self) {
        println!(" After quick_sort: {:?}", self.vector);
    }
}
        
