use rand::{rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let’s test your intuition! 🎯, Let’s Play the Number Guessing Game!");
    println!("🎲 The dice have rolled... the secret number lies between 1 and 100..");
    println!("🕵️ Your challenge: solve the mystery in as few attempts as possible!");
    println!("🌟 The fewer attempts, the higher your score!\n");

    let secret_number = rng().random_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("🔢 Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("❌ Failed to read input. Try again!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🚫  Invalid number. Please enter a number between 1 and 100.");
                continue;
            }
        };

        attempts += 1;

        println!("🥳 You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("☹️ Too Small! Try a higher number."),
            Ordering::Greater => println!(" 🙆🏻‍♀️Too Big! Try a lower number."),
            Ordering::Equal => {
                println!("\n Yayy! You nailed it in {attempts} attempt(s)!");

                let score = if attempts < 5 {
                    100 - (attempts * 10)
                } else if attempts < 10 {
                    50 - ((attempts - 5) * 5)
                } else {
                    0
                };

                println!("🏆 Final Score: {score}/100");
                println!("🔐And the magical number was {secret_number}.");
                println!("🎉🎮 GG! You crushed it!");
                println!("\n🚀 Thanks for playing. ");
                break;
            }
        }

        println!("–––––––––––––––––––––––––––––––––––––––––\n");
    }
}
