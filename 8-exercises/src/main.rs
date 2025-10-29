use clap::{command, Parser, Subcommand};

mod median;
mod mode;
mod piggy;
mod word_iter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
   commands: Command
}

#[derive(Subcommand, Clone)]
enum Command {
    Median { values: Vec<i32> },
    Mode { values: Vec<i32> },
    Piggy { words: Vec<String> },
}

fn main() {
    let cli = Cli::parse();
    match &cli.commands {
        Command::Median { values } => {
            match median::median(values) {
                Some(result) => {
                    println!("Median value is: {result}");
                },
                None => std::process::exit(exitcode::USAGE),
            }
        }
        Command::Mode { values } => {
            match mode::mode(values) {
                Some(result) => {
                    println!("Mode value is: {result}");
                },
                None => std::process::exit(exitcode::USAGE),
            }
        }
        Command::Piggy { words } => {
            println!("{}", piggy::piggy(&words.join(" ")));
        }
    }
    std::process::exit(exitcode::OK)
}
