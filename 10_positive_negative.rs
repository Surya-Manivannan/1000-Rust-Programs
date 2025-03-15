use std::io;

fn main() {

let mut input = String::new();
println!("Enter a number: ");
io::stdin().read_line(&mut input).unwrap();
let number: i32 = input.trim().parse().unwrap();

if number > 0 {
	println!("{number} is a Positive Integer");
} else if number < 0 {
	println!("{number} is a Negative Integer");
} else {
	println!("{number} is a Zero");
}
}
