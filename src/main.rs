use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("I have selected a random number between 1 and 100.");
    println!("You have 10 attempts to guess the number.");
    println!("Type 'exit', 'q', 'quit', 'end', or 'cancel' to give up and reveal the number.\n");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut counter = 10;

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        match guess.trim().to_lowercase().as_str() {
        "exit" | "q" | "quit" | "end" | "cancel" => {
            println!("Exiting the game. The secret number was {secret_number}.");
            break;
        }
        _ => {},
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number or a quit command.");
                continue;
            }
        };

        counter -= 1;
        println!("You have {counter} turns left.");

        if counter == 0 {
            println!("Game over! The secret number was {secret_number}.");
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}