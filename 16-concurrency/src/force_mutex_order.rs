use std::{
    ops::DerefMut,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
    thread,
    time::Duration,
};

struct ChainedMutexes<T>(Vec<Arc<Mutex<T>>>);
impl<T> ChainedMutexes<T> {
    fn new() -> ChainedMutexes<T> {
        ChainedMutexes(Vec::new())
    }

    fn and_then(mut self, mtx: Mutex<T>) -> ChainedMutexes<T> {
        self.0.push(Arc::new(mtx));
        self
    }

    fn lock(&self) -> Result<Vec<MutexGuard<'_, T>>, PoisonError<MutexGuard<'_, T>>> {
        let mut result: Vec<MutexGuard<'_, T>> = Vec::new();
        for m in &self.0 {
            result.push(m.lock()?)
        }
        Ok(result)
    }
}
impl<T> Clone for ChainedMutexes<T> {
    fn clone(&self) -> Self {
        Self(self.0.iter().map(Arc::clone).collect())
    }
}

pub fn run() {
    let mutexes = ChainedMutexes::new();
    let mutexes = mutexes.and_then(Mutex::new(0));
    let mutexes = mutexes.and_then(Mutex::new(1));

    eprintln!("Main thread acquiring locks");

    let lock = mutexes.lock().unwrap();

    eprintln!("Main thread spawning threads");

    let h1 = {
        let m = ChainedMutexes::clone(&mutexes);
        thread::spawn(move || {
            eprintln!("Thread 1 waiting for mutexes");
            let mut guard = m.lock().unwrap();
            eprintln!(
                "Thread 1 got lock for mutexes, contains: {:?}",
                guard.iter().map(|x| **x).collect::<Vec<i32>>()
            );
            eprintln!("Thread 1 yielding");
            thread::sleep(Duration::from_secs_f32(0.5));

            for i in &mut guard {
                *i.deref_mut() += 1;
            }

            eprintln!("Thread 1 dropping locks");
            drop(guard);

            eprintln!("Thread 1 yielding");
            thread::yield_now();

            eprintln!("Thread 1 is done")
        })
    };

    let h2 = {
        let m = ChainedMutexes::clone(&mutexes);
        thread::spawn(move || {
            eprintln!("Thread 2 waiting for mutexes");
            let mut guard = m.lock().unwrap();
            eprintln!(
                "Thread 2 got lock for mutexes, contains: {:?}",
                guard.iter().map(|x| **x).collect::<Vec<i32>>()
            );
            eprintln!("Thread 2 yielding");
            thread::sleep(Duration::from_secs_f32(0.5));

            for i in &mut guard {
                *i.deref_mut() += 10;
            }

            eprintln!("Thread 2 dropping locks");
            drop(guard);

            eprintln!("Thread 2 yielding");
            thread::yield_now();

            eprintln!("Thread 2 is done")
        })
    };

    eprintln!("Main thread releasing locks");
    drop(lock);

    eprintln!("Main thread yielding");
    thread::yield_now();

    eprintln!("Main thread waiting on children");
    h1.join().unwrap();
    h2.join().unwrap();

    eprintln!(
        "mutexes contains {:?}",
        mutexes
            .lock()
            .unwrap()
            .iter()
            .map(|x| **x)
            .collect::<Vec<i32>>()
    );

    eprintln!("Main thread is done");
}
