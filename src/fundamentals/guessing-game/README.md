# 🎯 Guess the Number - Rust Edition  

This is a simple Rust program that prompts the user to guess a number and echoes their input.  

## 🚀 How It Works  

1. The program prints **"Guess the number"** to the console.  
2. It prompts the user to enter a guess.  
3. The user input is read and stored in a variable.  
4. The input is then printed back to confirm what the user entered.  

## 📌 Code Overview  

```rust
use std::io;

fn main() {
    println!("Guess the number");

    println!("Please enter your guess....");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("You entered {}", guess);
}
```

## 📝 Topics Covered  

✔️ **Standard Input Handling (`io::stdin`)**  
✔️ **Mutable Variables (`mut`)**  
✔️ **Error Handling (`expect()`)**  
✔️ **String Manipulation (`String::new()`)**  
✔️ **Printing to Console (`println!`)**  

## ▶️ **How to Run**  

Make sure you have Rust installed. Then, compile and run:  

```sh
cargo run
```

Now, enter a number and see it printed back to you! 🎉  

---

# Resources
[Rust Handbook](https://doc.rust-lang.org/stable/book/title-page.html) | [Guessing Game](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html)