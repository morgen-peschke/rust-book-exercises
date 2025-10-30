use std::io::{self, Stdin};

#[derive(Debug)]
pub enum Stop {
    Quit,
    Failure(io::Error),
}

pub struct Prompt {
    nudge: String,
    quit: String,
    std_in: Stdin,
}

impl Prompt {
    pub fn new(nudge: &str, quit: &str) -> Prompt {
        let lower_quit = quit.to_ascii_lowercase();
        Prompt {
            nudge: format!("{nudge} or ('{}' to exit):", lower_quit),
            quit: lower_quit,
            std_in: io::stdin(),
        }
    }

    pub fn query(&self) -> Result<String, Stop> {
        println!("{}", self.nudge);
        let mut raw = String::new();
        let read_result = self.std_in.read_line(&mut raw).map(|_| raw.trim());

        match read_result {
            Ok(result) => {
                if result.to_ascii_lowercase() == self.quit {
                    Result::Err(Stop::Quit)
                } else {
                    Result::Ok(result.to_owned())
                }
            }
            Err(e) => Result::Err(Stop::Failure(e)),
        }
    }
}
