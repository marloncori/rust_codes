use std::collections::VecDeque;

fn get_num_power(vector: Vec<i32>) -> Vec<Option<i32>> {
    let powers = vector.iter()
                       .map(|&y| y.checked_pow(2))
                       .collect();
    powers
}

fn main() {
    let mut blank = "\t".to_string();
    let num_vec = vec![23, 45, 67, 17, 89, 92, 113];
    println!("\n\t This is your original num_vec: {:?}.", &num_vec);
    let output = get_num_power(num_vec);

    println!("\n\t This is the vector of numbers powered to 2: [");
    for num in output.clone() {
        match num {
            Some(val) => {
                      println!("{} {} ", blank, val);
                      blank += "\t";
                    },
            None      => println!(" There is no value in the variable!"),
        }
    }
    println!("\t].");

    println!("\n\t Now let us use a VecDeque for the first time!");
let mut my_deque = VecDeque::new();
       my_deque.extend(output.iter()
                                .map(|&y| {
                                    match y {
                                     Some(v) => v / 3,
                                     None    => 0, 
                                    } 
                                }));
   println!("\n\t New VecDeque: {:?}", my_deque);

   println!("\n\t You can now use collect() to make a String.");
   let chars = ['J', 'e', 's', 'u', 's', ' ', 'm', 'y', ' ', 'L', 'o', 'r', 'd'];
   let phrase: String = chars.iter()
                        .map(|&c| c as u8)
                        .map(|x| (x + 1) as char)
                        .collect();
   println!("\n\t\t Phrase: {:?}", phrase);
   let transcript: String = phrase.chars()
                            .map(|c| c as u8)
                            .map(|x| (x - 1) as char)
                            .collect();
 
   println!("\n\t\t Transcript: {:?}", transcript);

}
