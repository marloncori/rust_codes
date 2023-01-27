use insert_sort::algorithm;
use std::boxed::Box;

fn main() {
    let values = vec![56, 78, 34, 21, 90, 11, 5];    
    let mut sorter = Box::new(algorithm::InsertSort::new(values));
    let mut mode = 0;

    sorter.show_data(mode);
    sorter.start();

    mode = 1;
    sorter.show_data(mode);
    
}
