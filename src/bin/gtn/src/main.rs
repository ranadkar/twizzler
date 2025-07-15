use std::{
    cmp::{max, min},
    io::{Read, Write},
};

use embedded_io::ErrorType;
use rand::Rng;

const MIN_ANS: i32 = 1;
const MAX_ANS: i32 = 100;

struct TwzIo;

impl ErrorType for TwzIo {
    type Error = std::io::Error;
}

impl embedded_io::Read for TwzIo {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let len = std::io::stdin().read(buf)?;
        Ok(len)
    }
}

impl embedded_io::Write for TwzIo {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        std::io::stdout().write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        std::io::stdout().flush()
    }
}

fn main() {
    let mut lower = MIN_ANS;
    let mut upper = MAX_ANS;
    let mut guesses_remaining = 5;

    println!("I am thinking of a number between {MIN_ANS} and {MAX_ANS}.");

    let mut io = TwzIo;
    let mut buffer = [0; 1024];
    let mut history = [0; 1024];
    let mut editor = noline::builder::EditorBuilder::from_slice(&mut buffer)
        .with_slice_history(&mut history)
        .build_sync(&mut io)
        .unwrap();

    while guesses_remaining > 0 {
        // eprintln!("DEBUG {lower} {upper}");

        println!(
            "\n{guesses_remaining} {} left.",
            if guesses_remaining == 1 {
                "guess"
            } else {
                "guesses"
            }
        );

        let prompt = "Enter guess (or 'q' to quit): ";
        let guess_str = match editor.readline(prompt, &mut io) {
            Ok(line) => line.trim().to_string(),
            Err(_) => {
                println!("Error reading input");
                continue;
            }
        };

        if guess_str == "quit" || guess_str == "q" {
            let answer = rand::thread_rng().gen_range(lower..=upper);
            println!("Game over! The number was {answer}");
            break;
        }

        let guess: i32 = match guess_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between {MIN_ANS} and {MAX_ANS}.");
                continue;
            }
        };

        if guess < MIN_ANS || guess > MAX_ANS {
            println!("Invalid input. Please enter a number between {MIN_ANS} and {MAX_ANS}.");
            continue;
        }

        guesses_remaining -= 1;

        let thresh: i32 = 2_i32.pow(guesses_remaining);

        if guess - lower < thresh {
            lower = max(lower, guess + 1);
            println!("{guess} is too low!");
        } else if upper - guess < thresh {
            upper = min(upper, guess - 1);
            println!("{guess} is too high!");
        } else {
            let coinflip = rand::thread_rng().gen_range(0..=1);
            if coinflip == 0 {
                lower = max(lower, guess + 1);
                println!("{guess} is too low!");
            } else {
                upper = min(upper, guess - 1);
                println!("{guess} is too high!");
            }
        }

        if guesses_remaining == 0 {
            let answer = rand::thread_rng().gen_range(lower..=upper);
            println!("Game over! The number was {answer}");
            break;
        }
    }
}
