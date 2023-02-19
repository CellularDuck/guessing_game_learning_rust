use std::io;
use std::cmp::Ordering;
use rand::Rng;

// good luck
fn main() { 
    println!("guess the number!"); 
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret numero is {secret_number}");

    println!("input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    let guess: u32 = guess.trim().parse().expect("enter a number");

    println!("you guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("smol"),
        Ordering::Greater => println!("beeg"),
        Ordering::Equal => println!("yea youre correct"),
    }
}   
