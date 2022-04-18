use bubblesort::BubbleSort;

fn main() {
    let n1 = String::from("Marlon");
    let n2 = String::from("Zelia");
    let n3 = String::from("Laura");
    let n4 = String::from("Geysa"); 
    let n5 = String::from("Roberto");
    let n6 = String::from("Aline");

    let mut names = vec![n1, n2, n3, n4, n5, n6];
    println!("\n----> Unsorted names: {:?}", &names);
    
    let mut bubble_sort = BubbleSort::new(&mut names);
    bubble_sort.start_sorting();

    println!("\n----> Sorted names: {:?}", bubble_sort.get());
}
