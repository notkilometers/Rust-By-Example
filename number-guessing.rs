use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	let secret_numb : i8 = rand::thread_rng().gen_range(1..10);
	
	loop {
		println!("Please input your guess:");

		let mut guess : String = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess : i8 = match guess.trim().parse::<i8>() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&secret_numb) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {println!("You win"); break;}
		}
	}
}
