use std::{thread, time::Duration};

fn main() {
    let line = "lang:en=Jesus is my Lord!";
    let lang = "en";

    let v;
    println!("\n=========================================");
    wait();
    println!("    L I F E T I M E      E X A M P L E ");
    wait();
    println!("=========================================\n");
    wait();

    println!(" This is line you created: \n");
    wait();
    println!("    {}", line);
    wait();

    println!("  And this is the language: ");
    wait();
    println!(" language: {} [English]", lang);
    wait();
     {
        let p = format!("lang:{}=", lang);
        v = skip_prefix(line, p.as_str());
     }

    println!(" And now this is your V variable \n    after the function processing:");
    wait();
    println!("         {}", v);
    wait();
    println!("\n=========================================");
    wait();
    println!("       E N D     O F      E X A M P L E ");
    wait();
    println!("=========================================\n");
}

fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
#   line  
}
  
fn wait() {
   thread::sleep(Duration::from_secs(2));
}
