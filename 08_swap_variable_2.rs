use std::io;

fn main() {

let mut input = String::new();
println!("Enter a number1: ");
io::stdin().read_line(&mut input).unwrap();
let mut num1: i32 = input.trim().parse().unwrap();

let mut input = String::new();
println!("Enter a number2: ");
io::stdin().read_line(&mut input).unwrap();
let mut num2: i32 = input.trim().parse().unwrap();

println!("Before swapping, Number1: {num1}, Number2: {num2}");

num1 = num1 + num2;
num2 = num1 - num2;
num1 = num1 - num2;

println!("After swapping, Number1: {num1}, Number2: {num2}");

}
