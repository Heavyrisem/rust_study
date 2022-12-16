use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let max_try: i8 = 5;

    println!("Guess the number!");

    let mut tried = 0;
    loop {
        println!("Input your guess, {} try left", max_try - tried);
        tried += 1;

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim().eq("quit") {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        if max_try - tried <= 0 {
            println!("You lose, the number was {}", secret_number);
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Up"),
            Ordering::Greater => println!("Down"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
