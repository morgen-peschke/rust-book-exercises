use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Stdin};

const QUIT: &str = "quit";
const MAX: u32 = 20;

fn print_result(guess: u32, ord: Ordering) {
    println!(
        "You guessed {guess}, which was {}!",
        match ord {
            Ordering::Less => "too small",
            Ordering::Equal => "exactly right",
            Ordering::Greater => "too big",
        }
    );
}

fn main() {
    println!("Guess a number between 1 and {MAX}");
    let std_in: Stdin = io::stdin();
    let secret = rand::thread_rng().gen_range(1..=MAX);

    let read_guess = || -> String {
        let mut guess = String::new();

        std_in.read_line(&mut guess).expect("Failed to read line");

        guess.trim().to_owned()
    };

    loop {
        println!("Please input your guess (or '{QUIT}' to exit):");

        let guess = read_guess();

        let guess = match guess.parse::<u32>() {
            Ok(i) => i,
            Err(e) => {
                if guess.to_ascii_lowercase() == QUIT {
                    break;
                }
                println!("Not a number ({e}), try again.");
                continue;
            }
        };
        let cmp_res = guess.cmp(&secret);
        print_result(guess, cmp_res);
        if cmp_res.is_eq() {
            break;
        }
    }
}
