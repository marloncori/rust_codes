use quicksort::UnsortedList;

fn main() {
    
let zs = vec![32, 76, 43, 54, 21, 43, 23, 17, 98, 87, 54, 65, 98];
println!(" Unsorted vector: {:?}", &zs);

let mut unsorted_list = UnsortedList::new(zs);
unsorted_list.quick_sort(0, unsorted_list.vector.len()-1);
unsorted_list.show_sorted();


}
