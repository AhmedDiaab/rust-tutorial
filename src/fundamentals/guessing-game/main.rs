use rand::Rng;
use std::io;

fn main() {
    loop {
        let secret_number = rand::rng().random_range(1..=100);

        println!("The secret number is {secret_number}");
        println!("Guess the number");

        println!("Please enter your guess....");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
