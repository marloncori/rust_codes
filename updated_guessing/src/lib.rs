
pub mod game {

    pub struct Counter {
        pub attempts: i32,
    }

    impl Counter {
        pub fn new() -> Self {
            Counter {
              attempts: 0,
            }
        }
    }

    impl Iterator for Counter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let v = self.attempts;
            match v {
                v if v < 5 => {
                            self.attempts += 1;
                            Some(self.attempts)
                        },
                    _      => None,
            }
        }
    }

    pub struct Guess {
        pub value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Self {
            if value < 1 || value > 100 {
                panic!("\n\t Guess value must be a number \n\t between 1 and 100!");
            }
            Guess {
                value,
            }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

}