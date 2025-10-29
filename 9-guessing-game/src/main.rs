mod game_error;
mod guess;
mod prompt;
mod secret;

use game_error::GameError;
use guess::{Guess, IsSecret};
use prompt::Prompt;
use secret::Secret;
use std::{fmt::Display, process::ExitCode};

impl Display for IsSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IsSecret::Yes => f.write_str("exactly right"),
            IsSecret::Less => f.write_str("too small"),
            IsSecret::Greater => f.write_str("too big"),
        }
    }
}

fn main() -> ExitCode {
    let quit: String = "quit".to_owned();

    println!("Guess a number between {} and {}", secret::MIN, secret::MAX);
    let secret = Secret::random();

    let prompt = Prompt::new("Please input your guess", &quit);
    loop {
        let guess = prompt
            .query()
            .map_err(GameError::from)
            .and_then(|s| s.parse::<i32>().map_err(GameError::from))
            .and_then(|i| Guess::new(i).map_err(GameError::from))
            .map(|g| {
                let is_secret = g.is(&secret);
                (g, is_secret)
            });

        match guess {
            Ok((guess, is_secret)) => {
                println!("You guessed {guess}, which was {is_secret}!");
                if let IsSecret::Yes = is_secret {
                    break ExitCode::SUCCESS;
                } else {
                    continue;
                }
            }
            Err(game_error) => {
                println!("{game_error}");
                if let Some(exit_code) = game_error.exit_code() {
                    break exit_code;
                } else {
                    continue;
                }
            }
        }
    }
}
