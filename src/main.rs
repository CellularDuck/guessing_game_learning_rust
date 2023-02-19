use std::{io, thread, time};
use std::cmp::Ordering;
use rand::Rng;

// good luck
fn main() { 
    println!("guess the number!");

    //this generates the "secret" number 
    let secret_number = rand::thread_rng().gen_range(1..=100);
// use this for debugging lmfao    println!("the secret numero is {secret_number}");
    
    loop {
        // handles inputting
        println!("input your guess.");
    
        // assigns a mutable variable called guess with datatype string
        let mut guess = String::new();
    
        // this appends the input into guess 
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        // converts guess into unsigned 32 bit int 
        let guess: u32 = match guess.trim().parse() {
            // num is in std. thanks vscode for importing it for me
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("you guessed: {guess}");    

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("smol"),
            Ordering::Greater => println!("beeg"),
            Ordering::Equal => {
                println!("you win");
                println!("program will close in 5 seconds");
                thread::sleep(time::Duration::from_secs(5));
                break;
            }
        }
    }
}   