use insertsort::algorithm;

fn main() {
    let x_list = vec![87, 100, 12, 34, 21, 67, 33, 90, 5];
    println!(" --> Unsorted list: {:?}", &x_list);

    let mut unoredered_list = algorithm::Sortable::new(x_list);
    
    println!(" --> Sorted list: {:?}", unoredered_list.insert_sort());
    

    
}
