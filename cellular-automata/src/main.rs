mod collisions;
mod simple;

use std::error::Error;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run an elementary cellular automaton by Wolfram number
    Simple(SimpleArgs),
    /// Run a cellular automaton that simulates collisions
    Collider(ColliderArgs),
}

#[derive(Args)]
struct SimpleArgs {
    #[arg(
        short = 'r',
        long,
        help = "Wolfram code for the automaton",
        default_value_t = 154
    )]
    rule: u8,
    #[arg(
        short = 's',
        long = "state",
        help = "Initial state, true is any non-whitespace character"
    )]
    initial_state: Option<String>,
    #[arg(
        short = 'g',
        long = "generations",
        help = "Number of generations to run",
        default_value_t = 32
    )]
    generations: u32,
}

#[derive(Args)]
struct ColliderArgs {
    #[arg(
        short = 'b',
        long,
        help = "Do cells bounce off the ends of the field, or just come to a stop",
        default_value_t = false
    )]
    bounce: bool,
    #[arg(
        short = 'd',
        long = "damage",
        help = "Do cells that collide take damage, or are collisions 'winner takes all'",
        default_value_t = false
    )]
    partial_destroy: bool,
    #[command(flatten)]
    initial_state: ColliderStateArgs,
    #[command(flatten)]
    random_generation_args: ColliderStateRandomArgs,
    #[arg(
        short = 'g',
        long = "generations",
        help = "Number of generations to run, 0 means run until no more collisions are possible",
        default_value_t = 0
    )]
    generations: u32,
    #[arg(
        long,
        help = "Print each generation in a more verbose format, showing the weights as well as the directions",
        default_value_t = false
    )]
    debug: bool,
}

#[derive(Args, Clone)]
#[group(required = true, multiple = false)]
struct ColliderStateArgs {
    #[arg(
        short = 's',
        long = "state", 
        help = "Initial state, an empty cell is a '_', filled cells are numbers between 99 and 99, prefixed with a '-' for leftward movement or '+' for rightward movement and unprefixed if stationary. Each cell must be separated by a space",
        value_parser = collisions::State::from_string
    )]
    state_string: Option<collisions::State>,
    #[arg(
        required = true,
        short = 'r',
        long = "random",
        help = "Generate a random initial state"
    )]
    random: bool,
}

#[derive(Args, Clone)]
struct ColliderStateRandomArgs {
    #[arg(
        long = "rand-width",
        help = "Random initial state width, in cells. Ignored if state is specified",
        default_value_t = 80
    )]
    random_state_width: usize,
    #[arg(
        long = "rand-stationary",
        help = "Weighted chance of a stationary object. Ignored if state is specified",
        default_value_t = 20,
        value_parser = clap::value_parser!(u8).range(0..100)
    )]
    random_chance_stationary: u8,
    #[arg(
        long = "rand-left",
        help = "Weighted chance of a left-moving object. Ignored if state is specified",
        default_value_t = 20,
        value_parser = clap::value_parser!(u8).range(0..100)
    )]
    random_chance_left: u8,
    #[arg(
        long = "rand-right",
        help = "Weighted chance of a right-moving object. Ignored if state is specified",
        default_value_t = 20,
        value_parser = clap::value_parser!(u8).range(0..100)
    )]
    random_chance_right: u8,
    #[arg(
        long = "rand-empty",
        help = "Weighted chance of an empty cell. Ignored if state is specified",
        default_value_t = 40,
        value_parser = clap::value_parser!(u8).range(0..100)
    )]
    random_chance_empty: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    match Cli::parse().command {
        Commands::Simple(SimpleArgs {
            rule,
            initial_state,
            generations,
        }) => {
            let rule = simple::Wolfram::from_code(rule);
            let state = initial_state.unwrap_or_else(|| format!("{0}X{0}", " ".repeat(60)));
            let mut state = simple::State::from_string(&state);
            if generations == 0 {
                println!("{state}");
            }
            for _ in 0..generations {
                println!("|{state}|");
                state = state.next(&rule);
            }
            Ok(())
        }
        Commands::Collider(ColliderArgs {
            initial_state,
            generations,
            bounce,
            partial_destroy,
            debug,
            random_generation_args,
        }) => {
            let rule = collisions::Rule {
                bounce,
                partial_destroy,
            };
            let starting_state = initial_state.state_string.unwrap_or_else(|| {
                collisions::State::random(
                    random_generation_args.random_state_width,
                    random_generation_args.random_chance_stationary.into(),
                    random_generation_args.random_chance_left.into(),
                    random_generation_args.random_chance_right.into(),
                    random_generation_args.random_chance_empty.into(),
                )
            });
            let print = if debug {
                |state: &collisions::State| {
                    println!("{}", collisions::DebugOutput(state));
                }
            } else {
                |state: &collisions::State| {
                    println!("{state}");
                }
            };
            let mut state = starting_state.clone();
            if generations == 0 {
                print(&state);
                while let Some(next_state) = state.next(&rule) {
                    state = next_state;
                    print(&state);
                }
            } else {
                for _ in 0..generations {
                    print(&state);
                    match state.next(&rule) {
                        Some(new_state) => state = new_state,
                        None => break,
                    }
                }
            }
            println!(
                // The leading space is because clap has trouble dealing with
                // argument values that start with '-' and I don't want to
                // deal with that when copy-pasting initial states.
                "Initial state:\n' {}'",
                collisions::DebugOutput(&starting_state)
            );
            Ok(())
        }
    }
}
