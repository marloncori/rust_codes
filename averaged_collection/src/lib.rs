

pub mod calc {
   pub struct AveragedCollection {
      pub list: Vec<f32>,
      pub average: Option<f32>,
  }

  impl AveragedCollection {
      pub fn new(list: Vec<f32>) -> Self {
          AveragedCollection {
              list: list,
              average: None,
          }
      }

      pub fn add(&mut self, value: f32) {
          self.list.push(value);
          self.update_average();
      }

      pub fn remove(&mut self) -> Option<f32> {
          let result: Option<f32> = self.list.pop();
          match result {
              Some(v)  =>  {
                  self.update_average();
                  Some(v)
              },
              None => None,
          }
      }

      pub fn sum(&mut self) -> f32 {
          if !&self.list.is_empty() {
            match &self.list.pop() {
                Some(v) => v + self.sum(),
                None => 0.0,
           }
        } else {
            0.0
        }
      }

      pub fn subt(&mut self) -> f32 {
        if !&self.list.is_empty() {
           match &self.list.pop() {
              Some(v) => v - self.subt(),
              None => 0.0,
           }
        } else {
          -1.0
        }
      }

      pub fn mult(&mut self) -> f32 {
        if !&self.list.is_empty() {
           match &self.list.pop() {
              Some(v) => v * self.mult(),
              None => 1.0,
           }
        } else {
          0.0
        }
      }

      pub fn div(&mut self) -> f32 {
        if !&self.list.is_empty() {
           match &self.list.pop() {
              Some(v) => v / self.div(),
              None => 0.0,
           }
        } else {
          0.0
        }
      }

      pub fn reverse(&mut self) -> Vec<f32> {
          let mut nums: Vec<f32> = Vec::new();
          for _ in 0..self.list.len() {
              let value: f32 = match &self.list.pop(){
                 Some(v) => v.clone(),
                 None => 0.0,
              };
              nums.push(value);
           }
           return nums;
      }

      pub fn middle(&mut self) -> Result<f32, &str> {
        if self.list.len() % 2 != 0 {
            let even_size = self.list.len() - 1;
            let index = even_size / 2;
            return Ok(self.list[index]);
        }
        Err(" List has an even length, there is no middle.")
      }

      pub fn first_half(&mut self) -> Result<Vec<f32>, &str> {
        let mut n = Vec::new();
        if self.list.len() % 2 == 0 {
            let half = self.list.len() / 2;
            for x in 0..half {
                n.push(self.list[x])
            }
            return Ok(n);
        }
        Err(" List has an uneven length. Not possible to cut in halves.")
      }

      pub fn second_half(&mut self) -> Result<Vec<f32>, &str> {
        let mut n = Vec::new();
        if self.list.len() % 2 == 0 {
            let half = self.list.len() / 2;
            for _ in 0..half {
                let value = match self.list.pop(){
                        Some(v) => v,
                        None => 0.0
                };
                n.push(value)
            }
            return Ok(n);
        }
        Err(" List has an uneven length. Not possible to cut in halves.")
      }

      pub fn compare(&self, first: f32, second: f32) -> u8 {
         if first > second {
                return 1;
        }
        return 0;
      }

      pub fn sort(&mut self){
          let mut length = self.list.len();
          let last_elem = self.list.len() - 1;
          if length > 1{
            for elem in 0..last_elem {
                let next_elem = elem+1;
                if self.compare(self.list[elem], self.list[next_elem]) == 1 {
                    self.list[elem] = self.list[next_elem];
                    self.list[next_elem]  = self.list[elem];
                }
            }
            length-=1;
          }
      }

      pub fn show(&self) -> Vec<f32> {
          self.list.clone()
      }

      pub fn show_avg(&self) -> Option<f32> {
         self.average.clone()
      }
      
      pub fn update_average(&mut self) -> Option<f32> {
        self.average = Some((self.list.iter().sum::<f32>()) / 2.0);  
        self.average
      }
  }
}

