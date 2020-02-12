extern crate rand;
extern crate crossterm;

mod solarized;

use std::io::stdin;
use std::cmp::Ordering::{
    Less,
    Greater,
    Equal
};
use rand::Rng;
use crossterm::style::{
    Attribute,
    ResetColor,
    SetBackgroundColor,
    SetForegroundColor
};
use solarized::{
    clear,
    BACK,
    YELLOW,
    ORANGE,
    RED,
    MAGENTA,
    VIOLET,
    BLUE,
    CYAN,
    GREEN
};

fn main() {
    clear().unwrap();
    println!(
        "{}{}Please {}enter {}your {}name {}to {}save {}your {}win: {}",
        SetBackgroundColor(BACK),
        SetForegroundColor(YELLOW),
        SetForegroundColor(ORANGE),
        SetForegroundColor(RED),
        SetForegroundColor(MAGENTA),
        SetForegroundColor(VIOLET),
        SetForegroundColor(BLUE),
        SetForegroundColor(CYAN),
        SetForegroundColor(GREEN),
        ResetColor
    );
    let mut name = String::new();
    stdin().read_line(&mut name)
        .expect("Failed to read line.");
    clear().unwrap();
    println!(
        "{}{}Guess {}a {}five {}digit {}number {}now {}or {}else: {}",
        SetBackgroundColor(BACK),
        SetForegroundColor(YELLOW),
        SetForegroundColor(ORANGE),
        SetForegroundColor(RED),
        SetForegroundColor(MAGENTA),
        SetForegroundColor(VIOLET),
        SetForegroundColor(BLUE),
        SetForegroundColor(CYAN),
        SetForegroundColor(GREEN),
        ResetColor
    );
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
            Less => println!(
                "{}{}higher{}",
                SetBackgroundColor(BACK),
                SetForegroundColor(GREEN),
                ResetColor
                ),
            Greater => println!(
                "{}{}lower{}",
                SetBackgroundColor(BACK),
                SetForegroundColor(BLUE),
                ResetColor
                ),
            Equal => {
                println!(
                    "{}{}{}{}Well done {}{}{}, you matched in {}{}{} tries!{}",
                    SetBackgroundColor(BACK),
                    SetForegroundColor(CYAN),
                    Attribute::Bold,
                    Attribute::SlowBlink,
                    SetForegroundColor(VIOLET),
                    name.trim(),
                    SetForegroundColor(CYAN),
                    SetForegroundColor(VIOLET),
                    tries,
                    SetForegroundColor(CYAN),
                    ResetColor
                    );
                break;
            }
        }
    }
}
