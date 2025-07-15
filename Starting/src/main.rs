use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut input = String::new();

    println!("Enter a number between 1 and 10:");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let user_number: u32 = input.trim().parse().expect("Please enter a valid number");

    let random_number: u32 = rng.gen_range(1..=10);

    if user_number == random_number {
        println!("Congratulations! You guessed the correct number: {}", random_number);
    } else {
        println!("Sorry, the correct number was: {}", random_number);
    }
}
