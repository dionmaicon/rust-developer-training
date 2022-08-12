use std::fs::File;
use std::io::prelude::*;

fn read_file() {
    let mut file = File::open("./test.txt").expect("File not found.") ;
    let mut content = String::new();
    file.read_to_string( &mut content).expect("The file cannot be read.");
    println!("The content is: \n\n {}", content)
}

fn create_file() {
    let mut file = File::create("./test.txt").expect("File cannot be created.");
    file.write_all(b"File created").expect("File cannot be write");
}

fn main() {
       create_file();
       read_file();
}
