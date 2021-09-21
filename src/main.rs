use std::fs;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("reservas.txt").expect("could not read file");
    println!("file contents {}", contents);
}
