use ref_counter::nuc;
use std::rc::Rc;

fn main() {
    let name: Rc<String> = Rc::new(String::from("proglangs"));
    let ext: Rc<String> = Rc::new(String::from(".txt"));
    
    let my_file = nuc::MyFile::new(name, ext);
    my_file.show();

}
