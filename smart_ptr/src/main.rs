use smart_ptr::{List, Stack, Pile};
use std::boxed::Box;

fn main() {
    let nil = List::Empty;
    let null = Stack::Empty;
    let void = Pile::Empty;

    let lt = List::Cons(33.89, 
        Box::new(List::Cons(5.76, 
            Box::new(List::Cons(22.07, 
                Box::new(nil))))));
    println!("\n\t ==> {:#?}", lt);            
    
    let st = Stack::Cons("mpu6050", 
        Box::new(Stack::Cons("hc-sr04", 
            Box::new(Stack::Cons("hmc5883l", 
                Box::new(Stack::Cons("vl53l0x",
                   Box::new(null))))))));
    println!("\n\t\t ==> {:#?}", st);                
    
    let pl = Pile::Cons(true, 
        Box::new(Pile::Cons(false, 
           Box::new(Pile::Cons(true, 
                Box::new(void))))));
    println!("\n\t\t\t ==> {:#?}", pl);
}
