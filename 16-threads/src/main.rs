use clap::{Args, Parser};

mod channels_playground;
mod threads_playground;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Commands {
    Threads,
    Channels(ChannelArgs),
}

#[derive(Args, Debug)]
struct ChannelArgs {
    /// Number of producers
    #[arg(short, long)]
    producers: usize,
}

fn main() {
    match Commands::parse() {
        Commands::Threads => threads_playground::run(),
        Commands::Channels(ChannelArgs { producers }) => channels_playground::run(producers),
    }
}
