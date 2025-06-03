use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to number guessing game!");

    let target = rand::thread_rng().gen_range(1..=100);

    let mut attempts = 0;

    loop {
        let guess = match get_guess() {
            Some(num) => num,
            None => continue,
        };

        attempts += 1;

        if check_guess(guess, target) {
            println!("You have guessed {} times.", attempts);
            break;
        }
    }

    
}


fn get_guess() -> Option<u32> {
    println!("Guess a number!");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Failed to read input.");
        return None;
    }

    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Please input a valid number!");
            None
        }
    }
}


fn check_guess(guess: u32, target: u32) -> bool {
    match guess.cmp(&target) {
        std::cmp::Ordering::Less => {
            println!("Too small!");
            false
        } std::cmp::Ordering::Greater => {
            println!("Too big!");
            false
        } std::cmp::Ordering::Equal => {
            println!("Yes, the number is exactly {}!", target);
            true
        }
    }
}