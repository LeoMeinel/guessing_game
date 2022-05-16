/*
 * guessing_game is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/project/blob/main/LICENSE
 */
use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(0..101);
	println!("The secret number is: {}", secret_number);
	loop {
		println!("Please input your guess!");
		let mut guess = String::new();
		match io::stdin().read_line(&mut guess) {
			Ok(string) => string,
			Err(_) => {
				println!("{}", "WARNING: Failed to read line!".bright_red());
				continue;
			}
		};
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("{}", "WARNING: Please type a number!".bright_red());
				continue;
			}
		};
		println!("You guessed: {}", guess);
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("{}", "Too small!".yellow()),
			Ordering::Equal => {
				println!("{}", "Correct!".green());
				break;
			}
			Ordering::Greater => println!("{}", "Too big!".yellow()),
		}
	}
}
