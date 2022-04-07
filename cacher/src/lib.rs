
pub struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    pub calculation: T,
    pub value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v: u32 = (self.calculation)(arg);
                self.value = Some(v);
                v 
            },
        }
    }

}

