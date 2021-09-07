use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	// generate random numb
	let secret_numb : i8 = rand::thread_rng().gen_range(1..10);
	
	// loop until correct guess
	loop {
		println!("Please input your guess:"); // output 

		let mut guess : String = String::new(); // create string

		// read input
		io::stdin() 
			.read_line(&mut guess)
			.expect("Failed to read line");

		// parse to int
		let guess : i8 = match guess.trim().parse::<i8>() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		// match guess to number, output, break if correct
		match guess.cmp(&secret_numb) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {println!("You win"); break;}
		}
	}
}
