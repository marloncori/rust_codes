
pub mod online_shop {

  #[derive(PartialEq, Debug)]
  pub struct Shoe {
      pub size: u32,
      pub style: String,
  }

  #[derive(PartialEq, Debug)]
  pub struct Customer {
      pub age: u32,
      pub name: String,
      pub shoe_size: u32,
  }

  impl Customer {

      pub fn new(age: u32, name: String, shoe_size: u32) -> Self {
          Customer {
              age, name, shoe_size,
          }
      }

      pub fn buy_shoes(&self, pairs: Vec<Shoe>) -> Result<Vec<Shoe>, String> {
         let shoes = pairs.into_iter()
                    .filter(|x| x.size == self.shoe_size)
                    .collect::<Vec<Shoe>>();
         if shoes.is_empty(){
            return Err(String::from(" There are no shoes fitting the student's shoe size!"));
         }
          Ok(shoes)
      }
  }
}