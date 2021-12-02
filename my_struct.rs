//struct with tuple
struct Color<'a>(&'a u8, &'a u8, &'a u8);

//traditional struct
struct Point3D<'a> {
      x: &'a f32,
      y: &'a f32,
      z: &'a f32,
}

fn main() {
  
  let red = 98;
  let green = 120;
  let blue = 240;
  let c = Color(&red, &green, &blue);
  
  let new_x = 23.45;
  let new_y = 45.76;
  let new_z = 11.09;
  let pt = Point3D { x: &new_x, y: &new_y, z: &new_z, };
  
println!("\n++++++++++++++++++++++++++++++++++");
println!("+++ S T R U C T  E X A M P L E +++");
println!("++++++++++++++++++++++++++++++++++\n");

println!("   This is your created colour: ");
println!("   red: {}, green: {}, blue: {}", c.0, c.1, c.2);

println!("   And this is your created point 3D: ");
println!("   x: {},  y: {},  z: {}", *pt.x, *pt.y, *pt.z);

println!("\n++++++++++++++++++++++++++++++++++");
println!("+++ S T R U C T  E X A M P L E +++");
println!("++++++++++++++++++++++++++++++++++\n");
  
}
