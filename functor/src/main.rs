use functor::Functor;

fn main() {
    let val1: u16 = 32;
    let val2: u16 = 4;
    let val3 = &val1.fmap(|&v| (v / &val2));

    println!("\n The quotient of... \n\tval1: {} \n divided by... \n\tval2: {} \nequals --> \n\tval3: {}.", val1, val2, val3);
    
    let num1: i32 = -10;
    let num2: i32 = -2;
    let num3 = &num1.fmap(|&v| (v * &num2));

    println!("\n The product of... \n\tnum1: {} \n multiplied by... \n\tnum2: {} \nequals --> \n\tnum3: {}.", num1, num2, num3);

    let int1: u64 = 1446;
    let int2: u64 = 589;
    let int3 = &int1.fmap(|&v| (v - &int2));

    println!("\n The result of... \n\tint1: {} \n minus... \n\tint2: {} \nequals --> \n\tint3: {}.", int1, int2, int3);

}
