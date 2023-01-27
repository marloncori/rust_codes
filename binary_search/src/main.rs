use binary_search::binary;

fn main() {
    let numbers: Vec<u16> = vec![14, 2, 3, 12, 78, 20];
    let inf = 0;
    let sup = 6;
    let query = 12;
    let result = binary::search(numbers, inf, sup, query);
    println!(" query = {}, result = {}", query, result);
}
