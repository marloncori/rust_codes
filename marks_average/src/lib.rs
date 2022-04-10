
pub mod school {
    
    use std::num::ParseFloatError;

    pub fn adorn(line: &str, title: &str) {
        println!("{}", &line);
        println!("{}", title);
        println!("{}", line);
    }
    
    pub fn increment(ct: &mut u8) -> u8 {
        *ct += 1;
        *ct
    }
    
    pub fn show_marks(grades: &Vec<String>, mut counter: u8) {
        grades.iter().for_each(|grade| {
            let count = increment(&mut counter);
            println!("\t\t ==> {}: {}", count, grade);
        })
    }
    
    pub fn get_average(grades: &Vec<String>) -> Result<Option<f32>, ParseFloatError> {
        let sum: f32 = grades.iter()
                        .map(|str| str.trim()
                           .parse::<f32>().ok())
                             .flatten()
                               .fold(0.0, |x, y: f32| x +y);
        let average: f32 = sum / 2.0;
        Ok(Some(average))
    }
    
    pub fn show_average(grades: Vec<String>, avg: Option<f32>) {
        if let Some(value) = avg {
            println!("\n\t\t This is your average:");
            println!("\n\t\t ==> sum({:?}) / 2 \n\t\t\t{:?}", grades, avg);
        }
    }
}