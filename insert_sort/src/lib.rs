
pub mod algorithm {
    use std::boxed::Box;

    #[derive(Debug, Clone)]
    pub struct InsertSort {
        pub data: Vec<u32>,
    }

    impl InsertSort {
        pub fn new (data: Vec<u32>) -> Self {
            InsertSort {
                data,
            }
        }

        pub fn greater(&self, first: &u32, second: &u32) -> bool {
              first > second
        }

        pub fn start(&mut self){
            let size = self.data.len();
            let mut temp: u32;
            let mut k: Box<usize> = Box::new(0);
            for i in 0..size {
                temp = self.data[i];
                for j in (size-1)..0 {
                    if self.greater(&self.data[j-1], &temp) {
                        self.data[j] = self.data[j-1];
                        *k = j;
                    } 
                }
                self.data[*k] = temp;
            }
        }

        pub fn show_data(&self, mode: u32) {
            let mut word = String::new();
            match mode {
                0 => word = "unsorted".to_owned(),
                1 => word = "sorted".to_owned(),
                _ => word = "none".to_owned()
            }
            println!("\n This is the {} array:\n[", word);
            for i in &self.data {
                print!(" {:?} ", i);
            }
            println!(" ]\n");
        }
    }

}