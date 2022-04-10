use recursive_sum::operation;
use std::boxed::Box;

fn main() {
  let void = operation::NumberList::Empty;

  let list = operation::NumberList::Cons(32, 
      Box::new(operation::NumberList::Cons(45,
         Box::new(operation::NumberList::Cons(67,
            Box::new(operation::NumberList::Cons(58,
               Box::new(operation::NumberList::Cons(98,
                 Box::new(operation::NumberList::Cons(71,
                    Box::new(void))))))))))));

   let result = operation::sum_list(&list);
    println!("\n\t Orginal list: \n\t\t {:?} \n\t Number list sum = {:?}", &list, result);

   let state = operation::is_empty(&list);  
   println!("\n\t Is list empty now? {} \n\tChanged list: \n\t\t {:?}", state, list);
    
}
