extern crate rand;
extern crate crossterm;

mod solarized;

use std::io::{stdin, stdout};
use std::cmp::Ordering::{
    Less,
    Greater,
    Equal
};
use rand::Rng;
use crossterm::{
    ExecutableCommand,
    cursor,
    Result,
    terminal::{
        Clear,
        ClearType
    },
    style::{
        Attribute,
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor
    },
};

fn clear() -> Result<()> {
    stdout()
        .execute(Clear(ClearType::All))?
        .execute(cursor::MoveTo(0, 0))?;
    Ok(())
}

fn main() {
    clear().unwrap();
    println!("Enter your name: ");
    let mut name = String::new();
    stdin().read_line(&mut name)
        .expect("Failed to read line.");
    clear().unwrap();
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
            Less => println!("{}{}higher{}",
                SetBackgroundColor(solarized::BACK),
                SetForegroundColor(solarized::GREEN),
                ResetColor
                ),
            Greater => println!("{}{}lower{}",
                SetBackgroundColor(solarized::BACK),
                SetForegroundColor(solarized::BLUE),
                ResetColor
                ),
            Equal => {
                println!("{}{}{}{}Well done {}{}{}, you matched in {}{}{} tries!{}",
                    SetBackgroundColor(solarized::BACK),
                    SetForegroundColor(solarized::CYAN),
                    Attribute::Bold,
                    Attribute::SlowBlink,
                    SetForegroundColor(solarized::VIOLET),
                    name.trim(),
                    SetForegroundColor(solarized::CYAN),
                    SetForegroundColor(solarized::VIOLET),
                    tries,
                    SetForegroundColor(solarized::CYAN),
                    ResetColor
                    );
                break;
            }
        }
    }
}
