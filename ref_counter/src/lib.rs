
pub mod nuc {
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct MyFile {
        pub name: Rc<String>,
        pub extension: Rc<String>
    }

    impl MyFile {
        pub fn new(name: Rc<String>, extension: Rc<String>) -> Self {
            MyFile {
                name, extension
            }
        }

        pub fn show(&self){
            let line = "===================";
            println!("\n\t{}\n\t -- FILE -- \n\t{}\n\tName: {:?}\n\tExtension: {:?}\n\t{}", 
                line.clone(), &line, 
                 self.name.clone(), self.extension.clone(), line);
        }
    }
}