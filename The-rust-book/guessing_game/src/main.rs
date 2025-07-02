use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The scret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // it creates a new empty string
    // this is a mutable variable
    // rusts variable are unmutable by default
    // if we give a variable a value, then it cant change if its unmutable


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Low!"),
        Ordering::Greater => println!("Too High!"),
        Ordering::Equal => println!("You win!"),
    }

}
