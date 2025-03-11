# 🎯 Rust Guessing Game (Loop Version)  

This is an interactive number guessing game built in Rust. The game continuously generates a random number and allows the user to guess until they win.  

## 🚀 How It Works  

1. The program **loops indefinitely**, generating a new random number between **1 and 100** in each iteration.  
2. The user is prompted to guess the number.  
3. The program checks the guess and provides feedback:  
   - ✅ If the guess **matches** the secret number, the player wins! 🎉  
   - 📉 If the guess is **too low**, it prompts the user to guess higher.  
   - 📈 If the guess is **too high**, it prompts the user to guess lower.  
4. If the user enters invalid input (non-numeric), the program **continues** the loop without crashing.  
5. The loop continues until the user **guesses correctly**.  

## 📌 Code Overview  

```rust
use rand::Rng;
use std::io;

fn main() {
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Guess the number!");

        println!("Please enter your guess....");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Variable shadowing: 'guess' is declared again as a different type (u32)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Valid number input
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue; // Skip this iteration and prompt again
            }
        };

        println!("You entered {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Greater => println!("Too high! Try again."),
            std::cmp::Ordering::Less => println!("Too low! Try again."),
            std::cmp::Ordering::Equal => {
                println!("🎉 You win!");
                break; // Exit the loop when guessed correctly
            }
        }
    }
}
```

## 📝 Topics Covered  

✔️ **Random Number Generation (`rand::thread_rng().gen_range()`)**  
✔️ **Looping Until Win (`loop`, `break`, `continue`)**  
✔️ **User Input Handling (`io::stdin()`)**  
✔️ **Error Handling (`match guess.trim().parse()`)**  
✔️ **Mutable Variables (`mut`)**  
✔️ **Comparison (`cmp()`, `Ordering`)**  
✔️ **Variable Shadowing & Type Casting (`let guess = ...` inside the match statement)**  

## 🔍 **Understanding Variable Shadowing**  

In this program, **variable shadowing** is used when re-declaring the `guess` variable:

```rust
let mut guess = String::new(); // 'guess' is a String
...
let guess: u32 = match guess.trim().parse() { // 'guess' is now a u32
```

Even though the name remains the same, the `guess` variable is **re-declared** with a different type (`String` → `u32`). This is useful because we don't need a separate variable name for the parsed number.

## ▶️ **How to Run**  

1️⃣ **Install Rust** (if not already installed):  
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2️⃣ **Create a new Rust project** (or use an existing one):  
```sh
cargo new guessing_game
cd guessing_game
```

3️⃣ **Add the `rand` crate to your `Cargo.toml`:**  
```toml
[dependencies]
rand = "0.8"
```

4️⃣ **Run the game:**  
```sh
cargo run
```

Now, start guessing and enjoy the game! 🎮

---

# Resources
[Rust Handbook](https://doc.rust-lang.org/stable/book/title-page.html) | [Guessing Game](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html)