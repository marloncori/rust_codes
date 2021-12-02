//struct with tuple
struct Color(&u8, &u8, &u8);

//traditional struct
struct Point3D<&'a> {
      x: &f32,
      y: &f32,
      z: &f32,
}

fn main() {
  
  let red = 98;
  let green = 120;
  let blue = 240;
  let c = Color(&red, &green, &blue);
  
  let X = 23.45;
  let Y = 45.76;
  let Z = 11.09;
  let pt = Point { x: &X, y: &Y, z: &Z, }
  
println!("\n++++++++++++++++++++++++++++++++++");
println!("++++ S T R U C T  E X A M P L E +++");
println!("++++++++++++++++++++++++++++++++++\n");

println!("   This is your created colour: ");
println!("   red: {}, green: {}, blue: {}", c.0, c.1, c.2);

println!("   And this is your created point 3D: ");
println!("   x: {},  y: {},  z: {}", pt.x, pt.y, pt.z);

println!("\n++++++++++++++++++++++++++++++++++");
println!("++++ S T R U C T  E X A M P L E +++");
println!("++++++++++++++++++++++++++++++++++\n");
  
}
