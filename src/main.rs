// use rand::{rng, Rng};
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("Guess the number!");

//     let num = rng().random_range(1..=100);
//     let mut attempts = 0;

//     println!("The secret number is between 1 and 100.");

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Please enter a valid number.");
//                 continue;
//             }
//         };

//         attempts += 1;

//         match guess.cmp(&num) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");

//                 let score = if attempts < 5 {
//                     100 - (attempts * 10)
//                 } else if attempts < 10 {
//                     50 - ((attempts - 5) * 5)
//                 } else {
//                     0
//                 };

//                 println!("You guessed the number in {} attempts.", attempts);
//                 println!("Your score is: {}", score);
//                 println!("Thanks for playing!");
//                 break;
//             }
//         }
//     }
// }
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

// use std::cmp::Ordering;
// use std::io;

// use rand::{rng, Rng};

// fn main() {
//     println!("🎯 Welcome to the Ultimate Number Guessing Challenge!");
//     println!("I've secretly picked a number between 1 and 100.");
//     println!("Think you can guess it? Let the game begin!\n");

//     let secret_number = rng().random_range(1..=100);

//     loop {
//         println!("🔢 Enter your guess:");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Oops! Couldn't read your input.");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("⚠️  That doesn't look like a number. Try again!");
//                 continue;
//             }
//         };

//         println!("🎲 You guessed: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("📉 Too low! Aim higher."),
//             Ordering::Greater => println!("📈 Too high! Try a smaller number."),
//             Ordering::Equal => {
//                 println!("🎉 Bingo! You nailed it!");
//                 println!("🎊 The secret number was indeed {secret_number}. Well played!");
//                 break;
//             }
//         }

//         println!("–––––––––––––––––––––––––––––––––");
//     }

//     println!("🚀 Thanks for playing. Until next time, champion!");
// }
