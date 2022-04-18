

pub mod recursive {

    pub struct Recursion {
        pub list: Vec<i32>,
    }

    impl Recursion {
        pub fn add_list(&mut self, list: &mut Vec<i32>) -> i32 {
            if list.len() > 0 {
               match list.pop() {
                   Some(v) => v + self.add_list(list),
                   None => 0,
               }
            } else {
                return 0;
            }
        }
       
        pub fn mult_list(&mut self, list: &mut Vec<i32>) -> i32 {
            if list.len() > 0 {
                match list.pop() {
                   Some(v) => v * self.add_list(list),
                   None => 0,
                }
             } else {
                 return 0;
             }
        }

        pub fn subt_list(&mut self, list: &mut Vec<i32>) -> i32 {
            if list.len() > 0 {
                match list.pop() {
                   Some(v) => v - self.add_list(list),
                   None => 0,
                }
             } else {
                 return 0;
             }
        }

        pub fn div_list(&mut self, list: &mut Vec<i32>) -> i32 {
            if list.len() > 0 {
                match list.pop() {
                   Some(v) => v / self.div_list(list),
                   None => 0,
                }
             } else {
                 return 0;
             }
        }
    }
}
