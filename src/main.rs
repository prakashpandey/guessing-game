extern crate rand;
extern crate termion;

use std::io;
use std::io::Write;
use rand::Rng;
use termion::color;
use std::process;
//use termion::clear;

fn game(score: &mut i32, lower_limit: &i32, upper_limit: &i32) {
	let secret_number = rand::thread_rng().gen_range(*lower_limit, *upper_limit);
	
	print!("Input your guess:\t\t\t");
	io::stdout().flush().ok().expect("Could not flush stdio");
	
	let mut guess = String::new();

	io::stdin().read_line(&mut guess)
		.expect("Error reading line");

	let guess: i32 = guess.trim().parse()
		.expect("Please type a number");	
	
	print!("You guessed: {} \t\t\t\tThe secret number is: {} ", guess, secret_number);
	
	if guess == secret_number {
		print!("{green}\t\tYou won{reset}",
             		green   = color::Fg(color::Green),
             		reset = color::Fg(color::Reset));
		*score += 1;	
		
	} else {
		print!("{red}\t\tYou lose{reset}",
             		red   = color::Fg(color::Red),
             		reset = color::Fg(color::Reset));	
	}
	
}

fn message(lower_limit: &i32, upper_limit: &i32, max_attempt: &i32) {
	println!("\n\n-----------------------------------------------------------------------------------------------------------");
	println!("Guess the number[{} - {}] \t\tmax attempt: {}", *lower_limit, *upper_limit, *max_attempt);
}

fn clear() {
	std::process::Command::new("clear").status().unwrap().success();
	//println!("{}", clear::All);
}

fn play_game() {
	let upper_limit = 10;
	let lower_limit = 0;

	let mut score = 0;
	let max_attempt: i32 = 5;
	let mut i = 0;
	
	
	while i<max_attempt {
		clear();
		message(&lower_limit, &upper_limit, &max_attempt);
		print!("{red} Attempt: {i}{reset} \t\t\t\t{green}Score: {score}{reset}", 
				red   = color::Fg(color::Red), 
				i = i, 
				reset = color::Fg(color::Reset), 
				green   = color::Fg(color::Green), 
				score = score
			);
		println!("\n");
		game(&mut score, &lower_limit, &upper_limit);
		i += 1;
	}
	clear();
	println!("\n-------------------------------------------------------");
	println!("|Total attempt: {} \t\t\tTotal score: {}|", i, score);
	println!("-------------------------------------------------------");
}

fn main() {
	while true {
		play_game();
		print!("Press 1 to play again\t\t\t");
		io::stdout().flush().ok().expect("Could not flush stdio");
		let mut play_again = String::new();
		io::stdin().read_line(&mut play_again)
			.expect("Error reading line");
		let play: i32 = play_again.trim().parse().expect("Please type a number");	
		if play != 1 {
			process::exit(1);
		}

	}
}
