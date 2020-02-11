extern crate rand;
extern crate crossterm;

use std::io::{stdin, stdout, Write};
use std::cmp::Ordering::{
    Less,
    Greater,
    Equal
};
use rand::Rng;
use crossterm::{
    terminal::{
        Clear,
        ClearType
    },
    style::{
        Attribute,
        Color,
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor
    },
};

fn clear() {
    Clear(ClearType::All);
    let _ = stdout().flush();
}

fn main() {
    let back = Color::Rgb { r:7, g:54, b:66 };
    let _yellow = Color::Rgb { r:181, g:137, b:0 };
    let _orange = Color::Rgb { r:203, g:75, b:22 };
    let _red = Color::Rgb { r:211, g:1, b:2 };
    let _magenta = Color::Rgb { r:211, g:54, b:130 };
    let violet = Color::Rgb { r:108, g:113, b:196 };
    let blue = Color::Rgb { r:38, g:139, b:210 };
    let cyan = Color::Rgb { r:42, g:161, b:152 };
    let green = Color::Rgb { r:133, g:153, b:0 };
    clear();
    println!("Enter your name: ");
    let mut name = String::new();
    stdin().read_line(&mut name)
        .expect("Failed to read line.");
    clear();
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
                SetBackgroundColor(back),
                SetForegroundColor(green),
                ResetColor
                ),
            Greater => println!("{}{}lower{}",
                SetBackgroundColor(back),
                SetForegroundColor(blue),
                ResetColor
                ),
            Equal => {
                println!("{}{}{}{}Well done {}{}{}, you matched in {}{}{} tries!{}",
                    SetBackgroundColor(back),
                    SetForegroundColor(cyan),
                    Attribute::Bold,
                    Attribute::SlowBlink,
                    SetForegroundColor(violet),
                    name.trim(),
                    SetForegroundColor(cyan),
                    SetForegroundColor(violet),
                    tries,
                    SetForegroundColor(cyan),
                    ResetColor
                    );
                break;
            }
        }
    }
}
