use list_operations::recursive::Recursion;

fn main() {

    let mut deque = vec![1, 2, 3, 4, 5];
    let mut calc = Recursion {
        list: deque.clone(),
    };

    println!(" List: {:?} ", calc.list);
    println!(" Sum: {} ", calc.add_list(&mut deque));

    let mut deque = vec![1, 2, 3, 4, 5];
    println!(" Subt: {} ", calc.subt_list(&mut deque));

    let mut deque = vec![1, 2, 3, 4, 5];
    println!(" Prod: {} ", calc.mult_list(&mut deque));

    let mut deque = vec![1, 2, 3, 4, 5];
    println!(" Quot: {} ", calc.div_list(&mut deque));
}
