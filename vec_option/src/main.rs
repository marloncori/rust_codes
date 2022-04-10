
fn main() {
    let mut my_vector: Vec<u8> = vec![4, 5, 6, 7];
    show_vector(&mut my_vector);

    match get_muted_first(&mut my_vector){ 
        Some(v) => println!(" The first element was {} and now equals {}.", &v/2, v),
        None => println!(" No value has been found in num1 variable!"),
    }
    let mut my_vector2: Vec<u8> = vec![8, 9, 10, 12];
    show_vector(&mut my_vector2);

    match get_muted_first(&mut my_vector2){ 
        Some(v) => println!(" The first element was {} and now equals {}.", &v/2, v),
        None => println!(" No value has been found in num2 variable!"),
    }
}

fn show_vector(vector: &mut Vec<u8>) {
    println!("\n Vector = [");
        vector.iter().for_each(|x| print!("  {}, ", x));
    println!("]");
}

fn get_muted_first<'a>(vector: &mut Vec<u8>) -> Option<u8> {
    match vector.first() {
        Some(val) => Some(val + val),
        None => None,
    }
}