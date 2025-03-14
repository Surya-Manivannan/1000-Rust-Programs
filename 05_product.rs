use std::io;

fn main() {

let mut num1 = String::new();
println!("Enter a number1: ");
io::stdin().read_line(&mut num1).expect("Failed to read");
let num1: i32 = num1.trim().parse().expect("Failed to convert");

let mut num2 = String::new();
println!("Enter a number2: ");
io::stdin().read_line(&mut num2).expect("Failed to read");
let num2: i32 = num2.trim().parse().expect("Failed to convert");

println!("The product of two numbers: {}", num1*num2);

}
