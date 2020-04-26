use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secrect number is {}", secret_number);
    println!("Please input your guess!");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");

    let guess: u32 = guess.trim().parse().expect("Please input a number");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("Too big"),
    }
}
