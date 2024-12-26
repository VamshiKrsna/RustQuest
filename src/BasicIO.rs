use std::io;

fn main(){
    //Input : 
    println!("Enter Your Name : ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read Line");
    println!("Hello, {}!",name.trim());

    println!("Enter Your Age : ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read age");
    let age: u32 = age.trim().parse().expect("Please type in a number");
    println!("You are {} years old!",age);

    println!("Hello There");
    print!("Hello There");

    //Formatting Outputs : 
    let pi = 3.142345667786554;
    print!("The Value of Pi is : {:.2} \n",pi);
    print!("\n");
    print!("The Value of Pi is : {:.*}", 5, pi);
}