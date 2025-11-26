use std::fs;
use std::fs::File;

fn main() {
    File::Create("data.txt");
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
