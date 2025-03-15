use std::io;

fn main() {

let mut input = String::new();
println!("Enter a number: ");
io::stdin().read_line(&mut input).unwrap();
let number: i32 = input.trim().parse().unwrap();

if number % 2 == 0 {
	println!("{number} is a Even number");
} else {
	println!("{number} is a Odd number");
}
}

