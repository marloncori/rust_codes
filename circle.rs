struct Circle<'a>{
    x: &'a f64,
    y: &'a f64,
    rad: &'a f64,
}

impl Circle<'a> {
    fn new(new_x: &'a f64, new_y: &'a f64, new_rad: &'a f64) -> Circle {
        Circle {
           x: new_x,
           y: new_y,
           rad: new_rad,
        }
    }

    fn area(&self) -> f64 {
      std::f64::consts::PI * (self.rad * self.rad)
    }

    fn grow(&self, incr: &'a f64) -> Circle {
         Circle { x: self.x, y: self.y, rad: self.rad + icrement } 
    }

    fn reset_x(&mut self, new_x: &'a 64) -> f64 {
         self.x = new_x
    }

    fn reset_y(&mut self, new_y: &'a 64) -> f64 {
         self.y = new_y
    }

    fn reset_rad(&mut self, radius: &'a 64) -> f64 {
         self.rad = radius
    }
 }
  
fn main() {
  let new_x: f64 = 23.56;
  let new_y: f64 = 4.01;
  let new_rad: f64 = 45.978;
  let cc = Circle::new(&new_x, &new_y, &new_rad);
  
  println!("\n This is the Circle you created: ");
  println!(" x: {}, y: {}, radius: {}\n", *cc.x, *cc.y, *cc.rad);
  
  let my_x: f64 = 34.201;
  let my_y: f64 = 78.904;
  let my_rad: f64 = 123.57;
  
  cc.x = cc.reset_x(&my_x);
  cc.y = cc.reset_y(&my_y);
  cc.rad = cc.reset_rad(&my_rad);

  println!("\n\n This is the Circle after resetting its data: ");
  println!(" x: {}, y: {}, radius: {}\n", *cc.x, *cc.y, *cc.rad);
 
  let area = cc.area();
  println!("\n This is the Circle area: ");
  println!("      area: {}", area);
  
  let add_it: f64 = 45.978; 
  let increment = cc.grow(&add_it);
  println!("\n This is the resized radius: ");
  println!("         radius: {}\n", *cc.rad);
}
