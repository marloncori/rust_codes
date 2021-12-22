use std::{thread, time::Duration};

#[derive(Debug, PartialEq)]
struct Rectangle {
     width: u32,
     height: u32,
}

impl Rectangle {
   fn new(w: u32, h: u32) -> Self {
      Rectangle {
        width: w, height: h,
       }
     }

   fn square(size: u32) -> Rectangle {
      Rectangle {
         width: size,
         height: size,
       }
    }

   fn get_area(&self) -> u32 {
       self.width * self.height
    }

   fn get_inertia(&self) -> f32 {
      let self.width = self.width as f32;
      let self.height = self.height as f32;
      let h = f32::powf(3.0, self.height);
      (self.width * h) / 12.0
    }

   fn check_width(&self) -> bool {
      self.width > 0
   }

   fn check_height(&self) -> bool {
      self.height > 0
   }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn is_equal(&self, other: &Rectangle) -> bool {
    self.width == other.width && self.height == other.height
  }
}

fn main() {

   let rect1 = Rectangle::new(30, 50);
   let area1 = rect1.get_area();
   let inertia1 = rect1.get_inertia();

   let rect2 = Rectangle::new(70, 45);
   let area2 = rect2.get_area();
   let inertia2 = rect2.get_inertia();

   let rect3 = Rectangle::square(23);

   decorate();
   sleep();
   println!(" This is the first rectangle you created: \n\t{:?}", rect1);
   sleep();
   println!(" Does the rectangle have a nonzero width? {:?} !", rect1.check_width());
   sleep();
   println!("\tAnd does it have a nonzero height? {:?} !", rect1.check_height());
   sleep();
   println!("\n By multiplying the width {:?} and height {:?} \n\twe get the following area: {:?} cm2.", rect1.width, rect1.height, area1);
   sleep();
   println!("\nAnd by multiplying its base {:?} by the cubic root of its height {:?} and then \ndividing their product by 2 we get the following inertia \nin relation to an axis passing though its centroid: {:?}.", rect1.width, rect1.height, inertia1);
   wait();

   println!("\n\nThis is the second rectangle you created: \n\t{:?}", rect2);
   sleep();
   println!(" Does the rectangle have a nonzero width? {:?} !", rect2.check_width());
   sleep();
   println!("\tAnd does it have a nonzero height? {:?} !", rect2.check_height());
   sleep();
   println!("\n By multiplying the width {:?} and height {:?} \n\twe get the following area: {:?} cm2.", rect2.width, rect2.height, area2);
   sleep();
   println!("\nAnd by multiplying its base {:?} by the cubic root of its height {:?} and then \ndividing their product by 2 we get the following inertia \nin relation to an axis passing though its centroid: {:?}.", rect2.width, rect2.height, inertia2);
   wait();
   
   println!("\n Are Rectangle 1 and Rectangle 2 equal? {:?}.", rect1.is_equal(&rect2));
   sleep();
   println!("\t Can Rectangle 1 hold Rectangle 2 inside itself? {:?}.", rect1.can_hold(&rect2));
   wait();
   println!("\n\nThis is the SQUARE rectangle you created: \n\t{:?}\n", rect3);
   sleep();
   decorate();
}

fn sleep() {
  thread::sleep(Duration::from_secs(1));
}

fn wait() {
  thread::sleep(Duration::from_secs(3));
}

fn decorate() {
  let line = " =======================================================";
  let title = " *R E C T A N G L E S* - *S T R U C T*  &  *I M P L*";
  println!("\n{}", line);
  println!("{}", title);
  println!("{}", line);
}
