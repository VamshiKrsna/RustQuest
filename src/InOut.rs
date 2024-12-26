// use std::io::Write;

fn main(){
    println!("Hello World");
    let mut line = String::new();
    println!("Enter Your Name : ");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello, {}", line);
    println!("Change the Name : ");
    let b2 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Changed Name to : {}",line);
}