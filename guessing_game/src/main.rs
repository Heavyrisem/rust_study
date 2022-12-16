extern crate rand; // ??

use core::num::dec2flt::number::Number;
use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number is: {}", secret_number);

    println!("Input your guess.");

    let mut guess = integer::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Down"),
        Ordering::Greater => println!("Up"),
        Ordering::Equal => println!("You win")
    }
}