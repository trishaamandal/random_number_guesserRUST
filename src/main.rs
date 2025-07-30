use rand::{rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let num = rng().random_range(1..=100);
    let mut attempts = 0;

    println!("The secret number is between 1 and 100.");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");

                let score = if attempts < 5 {
                    100 - (attempts * 10)
                } else if attempts < 10 {
                    50 - ((attempts - 5) * 5)
                } else {
                    0
                };

                println!("You guessed the number in {} attempts.", attempts);
                println!("Your score is: {}", score);
                println!("Thanks for playing!");
                break;
            }
        }
    }
}
