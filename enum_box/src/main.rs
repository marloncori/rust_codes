use enum_box::{Point, List};

fn main() {
    let p1 = Point::new(3, 5);
    let p2 = Point::new(7, 1);
    let p3 = Point::new(4, 2);
    let p4 = Point::new(9, 14);

    let point_list = List::Cons(p1.clone(), Box::new(List::Cons(p2.clone(), Box::new(
        List::Cons(p3.clone(), Box::new(List::Cons(p4.clone(), Box::new(List::Empty)))
    )))));
    
    let points: Vec<Point> = vec![p1, p2, p3, p4];
    for point in points{
        println!(" {}", point);
    }
    
    println!("\n\t {:#?}", point_list);
}
