use std::fs::File;
use std::io::{Read, Write, BufReader, BufRead};

fn main() {
    // Reading File Contents
    let mut file = File::open("hello.txt").expect("Failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed");
    print!("File Contents : {}", contents);

    // IO With Buffers
    let file = File::open("hello.txt").expect("Failed");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        print!("\n{}", line.expect("Failed"));
    }

    // Write to file
    let mut file = File::create("hello.txt").expect("Failed");
    file.write_all(b"You are so Screwed").expect("Failed");

    // Read the file again
    let file = File::open("hello.txt").expect("Failed");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        print!("\n {}", line.expect("Failed"));
    }
}
