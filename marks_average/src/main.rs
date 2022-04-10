use std::io;
use marks_average::school;

fn main() {
    let ln = "\n\t------------------------------------";
    let tl = "\t----M A R K   A V E R A G E ----";
    let mut counter: u8 = 0;
    let mut grades: Vec<String> = Vec::new();

    school::adorn(ln, tl);
    println!("\n\t How many grades will you enter?");
    let mut total = String::new();
    io::stdin().read_line(&mut total).expect(" An unexpected error happened!");
    let total: u8 = total.trim().parse().unwrap();
    
    for g in 1..=total {
        match g {
            1 => { 
                print!("\t => {}st grade: ", g);
                println!();
            },
            2 => { 
                print!("\t => {}nd grade: ", g);
                println!();
            },
            3 => { 
                print!("\t => {}rd grade: ", g);
                println!();
            },
            g if g==total => { 
                print!("\t => Last grade: ");
                println!();
            },
            _ => { 
                print!("\t => {}th grade: ", g);
                println!();
            },
        }
        let mut grade = String::new();
        io::stdin().read_line(&mut grade).expect(" Error reading user input...");
        grades.push(grade);
    }
    school::show_marks(&grades, counter);
    let avg: Option<f32> = school::get_average(&mut grades).unwrap();
    
    school::show_average(grades, avg);
    school::adorn(ln, tl);
}

