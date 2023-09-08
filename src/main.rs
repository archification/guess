extern crate rand;
extern crate crossterm;

mod common;
mod solarized;

use std::io::stdin;
use std::cmp::Ordering::{
    Less,
    Greater,
    Equal
};
use rand::Rng;
use solarized::{
    print_colored, print_fancy,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    BOLD, UNDERLINED, ITALIC
};
use common::clear;

fn main() {
    clear();
    print_colored(
        &["Please ", "enter ", "your ", "name ", "to ", "save ", "your ", "score: "],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    let mut name = String::new();
    stdin().read_line(&mut name)
        .expect("Failed to read line.");
    clear();
    print_colored(
        &["Guess ", "a ", "five ", "digit ", "number ", "now ", "or ", "else "],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA]
    );
    let secret_number = rand::thread_rng().gen_range(10000..99999);
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
            Less => print_colored(
                &["higher"],
                &[GREEN]
            ),
            Greater => print_colored(
                &["lower"],
                &[GREEN]
            ),
            Equal => {
                print_fancy(&[
                    ("Well done ", CYAN, vec![BOLD, ITALIC]),
                    (name.trim(), VIOLET, vec![BOLD, UNDERLINED]),
                    (", you matched in ", CYAN, vec![BOLD, ITALIC]),
                    (&format!("{}", tries), VIOLET, vec![BOLD, UNDERLINED]),
                    (" tries!", CYAN, vec![BOLD, ITALIC]),
                ]);
                break;
            }
        }
    }
}
