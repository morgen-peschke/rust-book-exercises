use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn run(producers: usize) {
    let (tx, rx) = mpsc::channel();

    let spawn = |i: usize| {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("--> {i} starting");
            let limit: usize = rand::random_range(0..=i);
            for c in ('a'..='z').take(5 + limit) {
                tx.send((i, c)).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
            println!("--> {i} done");
            // Without this, tx doesn't get dropped for some reason, and
            // the receiver never exits
            drop(tx);
        })
    };

    let handles: Vec<JoinHandle<()>> = (0..producers).map(spawn).collect();
    // Without this, tx doesn't get dropped until the end of the method, and
    // the receiver never exits
    drop(tx);

    println!("<-- Starting");
    loop {
        match rx.try_recv() {
            Ok((i, s)) => println!("<-- Received from {i}: {s}"),
            Err(mpsc::TryRecvError::Empty) => {
                println!("<-- Empty, sleeping 0.5 seconds");
                thread::sleep(Duration::from_secs_f32(0.5));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("<-- Sender disconnected");
                break;
            }
        }
    }
    println!("<-- Done");

    for h in handles {
        h.join().unwrap()
    }
}
