extern crate rand;
extern crate termion;

use std::io::{stdin, stdout, Write};
use std::cmp::Ordering::{Less, Greater, Equal};
use rand::Rng;
use termion::{clear};

fn main() {
    println!("{}", clear::All);
    let _ = stdout().flush();
    println!("Enter your name: ");
    let mut name = String::new();
    stdin().read_line(&mut name)
        .expect("Failed to read line.");
    println!("Guess a five digit number.");
    let secret_number = rand::thread_rng().gen_range(10000, 99999);
    let mut tries = 0;

    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        tries += 1;

        match guess.cmp(&secret_number) {
            Less => println!("higher"),
            Greater => println!("lower"),
            Equal => {
                println!("Well done {}, you guessed my number in {} tries!", name.trim(), tries);
                break;
            }
        }
    }
}
