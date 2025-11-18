use std::{
    sync::{Arc, Mutex, MutexGuard, PoisonError, TryLockError},
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

type Guards<'a, A, B> = (MutexGuard<'a, A>, MutexGuard<'a, B>);
type LockFailure<'a, A> = PoisonError<MutexGuard<'a, A>>;
type Failures<'a, A, B> = Either<LockFailure<'a, A>, LockFailure<'a, B>>;

fn acquire_both<'a, A, B>(
    tag: &str,
    a: &'a Mutex<A>,
    b: &'a Mutex<B>,
) -> Result<Guards<'a, A, B>, Failures<'a, A, B>> {
    loop {
        let mg_a = match a.lock() {
            Ok(mg) => mg,
            Err(e) => break Err(Either::Left(e)),
        };
        eprintln!("{tag} acquired lock A, sleeping before acquiring lock B");
        thread::sleep(Duration::from_secs_f32(0.5));
        match b.try_lock() {
            Ok(mg_b) => {
                eprintln!("{tag} acquired locks A and B");
                break Ok((mg_a, mg_b));
            }
            Err(tle) => match tle {
                TryLockError::WouldBlock => {
                    eprintln!(
                        "{tag} possible deadlock identified, dropping lock A and sleeping before trying again"
                    );
                    drop(mg_a);
                    thread::sleep(Duration::from_secs_f32(0.5));
                    continue;
                }
                TryLockError::Poisoned(pe) => break Err(Either::Right(pe)),
            },
        }
    }
}

pub fn run() {
    let mut_one = Arc::new(Mutex::new(0));
    let mut_two = Arc::new(Mutex::new(1));

    eprintln!("Main thread acquiring locks");

    let lock_one = mut_one.lock().unwrap();
    let lock_two = mut_two.lock().unwrap();

    eprintln!("Main thread spawning threads");

    let spawn = |tag: &str,
                 inc_by: i32,
                 first_mutex: &Arc<Mutex<i32>>,
                 second_mutex: &Arc<Mutex<i32>>|
     -> JoinHandle<()> {
        let m_1 = Arc::clone(first_mutex);
        let m_2 = Arc::clone(second_mutex);
        let t_tag = tag.to_owned();
        thread::spawn(move || {
            eprintln!("{t_tag} waiting for mutexes");
            let (mut guard_1, mut guard_2) = acquire_both(&t_tag, &m_1, &m_2).unwrap();
            eprintln!("{t_tag} got locks containing: {} & {}", guard_1, guard_2);
            eprintln!("{t_tag} yielding");
            thread::sleep(Duration::from_secs_f32(0.5));

            *guard_1 += inc_by;
            *guard_2 += inc_by;

            eprintln!("{t_tag} dropping locks");
            drop(guard_1);
            drop(guard_2);
            eprintln!("{t_tag} yielding");
            thread::yield_now();

            eprintln!("{t_tag} is done")
        })
    };

    let h1 = spawn("-", 10, &mut_one, &mut_two);
    let h2 = spawn("*", 100, &mut_two, &mut_one);

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
