use std::io;

fn main() {

let mut input = String::new();
println!("Enter a Divedend: ");
io::stdin().read_line(&mut input).expect("Failed to read");
let divedend:i32 = input.trim().parse().expect("Failed to convert");

input.clear();

let mut input = String::new();
println!("Enter a Divisor: ");
io::stdin().read_line(&mut input).expect("Failed to read");
let divisor: i32 = input.trim().parse().expect("Failed to convert");

let quotient = divedend / divisor;

println!("Quotient is {}", quotient);

}
