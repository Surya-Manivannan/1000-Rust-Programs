use std::io;

fn main() {

let mut input = String::new();
println!("Enter a number: ");
io::stdin().read_line(&mut input).unwrap();
let mut num1: i32 = input.trim().parse().unwrap();

let mut input = String::new();
println!("Enter a number: ");
io::stdin().read_line(&mut input).unwrap();
let mut num2: i32 = input.trim().parse().unwrap();

println!("Before swapping, number1: {num1}, number2: {num2}");

let temp = num1;
num1 = num2;
num2 = temp;


println!("After swapping, number1: {num1}, number2: {num2}"); 
}
