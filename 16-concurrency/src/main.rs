use clap::{Args, Parser};

mod acquire_all_mutexes;
mod channels_playground;
mod deadlocks;
mod force_mutex_order;
mod threads_playground;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Commands {
    Threads,
    Channels(ChannelArgs),
    ChannelsExample,
    Deadlocked,
    ForcedMutexOrder,
    AcquireAllMutexes,
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
        Commands::ChannelsExample => channels_playground::run_example(),
        Commands::Channels(ChannelArgs { producers }) => channels_playground::run(producers),
        Commands::ForcedMutexOrder => force_mutex_order::run(),
        Commands::Deadlocked => deadlocks::run(),
        Commands::AcquireAllMutexes => acquire_all_mutexes::run(),
    }
}
