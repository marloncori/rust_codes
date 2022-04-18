
pub struct BubbleSort<'a> {
    pub collection: &'a mut Vec<String>,
}

impl<'a> BubbleSort<'a> {
    pub fn new(collection: &'a mut Vec<String>) -> Self {
        BubbleSort {
            collection,
        }
    }

    pub fn get(&self) -> Vec<String>{
        self.collection.clone()
    }

    fn compare(this: &String, other: &String) -> u8 {
        if this.as_bytes()[0] > other.as_bytes()[0]{
            return 1;
        } else {
            return 0;
        }
    }

    pub fn start_sorting(&mut self){
       let mut length = self.collection.len();
       let last_elem = length - 1;
       let one_elem_left = 1;

       while length > one_elem_left {
           for elem in 0..last_elem {
               if BubbleSort::compare(&self.collection[elem],
                  &self.collection[elem+1]) == 1 {
                     let (first, second) = (self.collection[elem+1].clone(), self.collection[elem].clone());
                           self.collection[elem] = first;
                           self.collection[elem+1] = second;
                }
           }
            length -= 1;        
        }
    }
}

