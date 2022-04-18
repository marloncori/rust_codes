use averaged_collection::calc;

fn main() {
    let float_deque: Vec<f32> = vec![2.0, 3.4, 5.7, 8.9, 1.9];
    println!(" Orginal list : {:?}", &float_deque);
    let mut collection = calc::AveragedCollection::new(float_deque);
    println!("Hello, world!");
    collection.add(7.3);
    println!(" List : {:?}", collection.show());

    collection.remove();
    println!(" List : {:?}", collection.show());


    match collection.show_avg() {
         Some(value) => println!(" List average: {:?}", value),
         None => println!(" List has no average value!"),
    }
    
    match collection.middle() {
        Ok(v) => println!(" Middle of list : {:?}", v),
        Err(err) =>  println!(" {}", err), 
    }
    
    collection.remove();
    println!(" List : {:?}", collection.show());
    
    collection.add(8.41);
    collection.add(3.12);

    println!(" List : {:?}", collection.show());
    //match collection.second_half() {
    //    Ok(v) => println!(" Second half of list : {:?}", v),
    //    Err(err) =>  println!(" {}", err), 
    //}

    collection.sort();
    println!(" SORTED List : {:?}", collection.show());

    match collection.first_half() {
        Ok(v) => println!(" Second half of list : {:?}", v),
        Err(err) =>  println!(" {}", err), 
    }

    println!(" Reversed list : {:?}", collection.reverse());
    //println!(" Sum of list : {:?}", collection.sum());
    
    
}
