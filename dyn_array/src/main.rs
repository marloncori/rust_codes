use dyn_array::container;
use std::boxed::Box;

fn main() {
    
    let v = Some(vec![200u64, 170, 150, 130, 110, 100, 50, 30, 10, 5, 3, 1]);
    let buf = Box::new(&v[..]);
    let dyn_arr = 
        container::TimestampSaver::new(buf);    

    println!(" Value: {:?}", dyn_arr.at(4));
}
