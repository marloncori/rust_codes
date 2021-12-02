struct Rectangle<T> {
    width: T,
    height: T,
}

trait Rectangle<T> {
    fn is_square(&self) -> bool;
}

impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
       self.width == self.height
    }
  }
  
fn main() {
    let rect = Rectangle {
         width: 2.34,
         height: 2.34,
        }

   let rect2 = Rectangle {
         width: 22.34,
         height: 2.32,
        }
  
   println!("\nThis is your first rectangle:");
   println!("  width: {}, height: {}\n", rect.width, rect.height);
   println!("Is it a square rectangle? {} \n", assert!(rect.is_square()));
  
   println!("\nAnd this is your second rectangle:");
   println!("  width: {}, height: {}\n", rect2.width, rect2.height);
   println!("And is it a square rectangle? {}", assert!(rect2.is_square()));
}
