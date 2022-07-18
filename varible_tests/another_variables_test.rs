fn main() {
	let b = 5;
	let b = b + 1;
	{
		let b = b * 2;
		println!("The value of b in this scope is {}", b);
	}
	{
		let b = b + 3;
		println!("the value of b in the second scope is {}", b);
	}
	println!("The value of b is{}", b);
}