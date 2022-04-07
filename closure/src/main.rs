
fn modify_vector(vector: &[i16]) -> Vec<i16> {
    let increment = vector.iter()
                    .map(|x| x + 2);
    increment.collect::<Vec<i16>>()
}

fn get_uneven_nums(vector: &[i16]) -> Vec<i16> {
    let mut inner_vec: Vec<i16> = Vec::new();
    inner_vec.extend(vector.iter().filter(|y| *y % 2 != 0));
    inner_vec
}


fn main() {

    let container: Vec<i16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let num_vector = &container[..];

    println!("\n\t This is the vector you created: \n\t {:?}.", num_vector);

    let increment = modify_vector(container.as_slice());
    let copy_vec = increment.clone();
    println!("\n\t And here you are the slice incremented by 2: \n\t [{:?}]", increment);            
    
    let unevens = get_uneven_nums(copy_vec.as_slice());
    println!("\n\t These are your even numbers from copy_vec: \n\t {:?}", unevens);
    
    let inc_even = container.iter()
                            .map(|i| i + 5)
                            .filter(|z| *z % 2 != 0)
                            .collect::<Vec<i16>>();
    
    println!("\n\t And this is the vector of uneven numbers \n         incremented by five: \n\t {:?}", inc_even);

}
