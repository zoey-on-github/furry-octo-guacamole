use std::io;

fn main() {
	println!("Guess the number!:");

	println!("Input your guess");

	let mut guess = String::new();
	let guess: u32 = guess.trim().parse().expect("Please type a number!");

	io::stdin()
		.read_line(&mut guess)	
		.expect("failed to read line");

	println!("You guessed {}!", guess)
}