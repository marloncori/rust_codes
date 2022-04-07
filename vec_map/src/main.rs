fn main() {
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let mut c = 0;

    println!("\n\t This is your reverserd \'letters\' vector:");
    for pair in letters.into_iter()
                 .map(|lt| { c += 1; (lt, c)})
                 .rev() {
     println!("\n\t {:?}", pair);
    }  

    let my_vec: Vec<i32> = vec![21, 37, 85, 47];
    println!("\n\t This was your original my_vec: \n\t {:?}", my_vec.clone());
    let new_vec = my_vec.into_iter()
                            .map(|x| x + 13)
                            .rev().collect::<Vec<i32>>();
   println!("\n\t This is your modified vector: \n\t {:?}", new_vec);
                            
}
