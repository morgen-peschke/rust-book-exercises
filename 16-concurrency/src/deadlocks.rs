use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn run() {
    let mut_one = Arc::new(Mutex::new(0));
    let mut_two = Arc::new(Mutex::new(1));

    eprintln!("Main thread acquiring locks");

    let lock_one = mut_one.lock().unwrap();
    let lock_two = mut_two.lock().unwrap();

    eprintln!("Main thread spawning threads");

    let h1 = {
        let m_1 = Arc::clone(&mut_one);
        let m_2 = Arc::clone(&mut_two);
        thread::spawn(move || {
            eprintln!("Thread 1 waiting for mutex 1");
            let mut guard_1 = m_1.lock().unwrap();
            eprintln!("Thread 1 got lock for mutex 1, contains: {}", guard_1);
            eprintln!("Thread 1 yielding");
            thread::sleep(Duration::from_secs_f32(0.5));

            eprintln!("Thread 1 waiting for mutex 2");
            let mut guard_2 = m_2.lock().unwrap();
            eprintln!("Thread 1 got lock for mutex 2, contains: {}", guard_2);
            eprintln!("Thread 1 yielding");
            thread::sleep(Duration::from_secs_f32(0.5));

            *guard_1 += 1;
            *guard_2 += 1;

            eprintln!("Thread 1 dropping locks");
            drop(guard_1);
            drop(guard_2);
            eprintln!("Thread 1 yielding");
            thread::yield_now();

            eprintln!("Thread 1 is done")
        })
    };

    let h2 = {
        let m_1 = Arc::clone(&mut_one);
        let m_2 = Arc::clone(&mut_two);
        thread::spawn(move || {
            eprintln!("Thread 2 waiting for mutex 2");
            let mut guard_2 = m_2.lock().unwrap();
            eprintln!("Thread 2 got lock for mutex 2, contains: {}", guard_2);
            eprintln!("Thread 2 yielding");
            thread::sleep(Duration::from_secs_f32(0.5));

            eprintln!("Thread 2 waiting for mutex 1");
            let mut guard_1 = m_1.lock().unwrap();
            eprintln!("Thread 2 got lock for mutex 1, contains: {}", guard_1);
            eprintln!("Thread 2 yielding");
            thread::sleep(Duration::from_secs_f32(0.5));

            *guard_1 += 10;
            *guard_2 += 10;

            eprintln!("Thread 2 dropping locks");
            drop(guard_1);
            drop(guard_2);
            eprintln!("Thread 2 yielding");
            thread::yield_now();

            eprintln!("Thread 2 is done")
        })
    };

    eprintln!("Main thread releasing locks");
    drop(lock_one);
    drop(lock_two);
    eprintln!("Main thread yielding");
    thread::yield_now();

    eprintln!("Main thread waiting on children");
    h1.join().unwrap();
    h2.join().unwrap();

    eprintln!("mut_one contains {}", mut_one.lock().unwrap());
    eprintln!("mut_two contains {}", mut_two.lock().unwrap());

    eprintln!("Main thread is done");
}
