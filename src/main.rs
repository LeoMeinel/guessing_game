/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::cmp::Ordering;
use std::io;

use colored::Colorize;
use rand::Rng;

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
