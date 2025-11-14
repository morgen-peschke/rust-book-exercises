use std::{
    fmt::{Debug, Write},
    thread,
};

#[derive(Copy, Clone)] // Needed for non-primitive types
struct Test(char, char, char);
impl Debug for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0)?;
        f.write_char(self.1)?;
        f.write_char(self.2)
    }
}

pub fn run() {
    custom_struct();
    println!();
    primitive_value();
}

fn custom_struct() {
    println!("    ==== Custom Struct ====");
    let mut n = Test(' ', ' ', ' ');
    let t = thread::spawn(move || {
        n.1 = 'O';
        thread::yield_now();
        let inner_handle = thread::spawn(move || {
            n.2 = 'I';
            thread::yield_now();
            println!("Inner thread n: {n:?}")
        });
        thread::yield_now();
        println!("Outer thread n: {n:?}");
        thread::yield_now();
        inner_handle
    });
    n.0 = 'M';

    t.join().and_then(|h| h.join()).unwrap();

    println!("Main  thread n: {n:?}")
}

fn primitive_value() {
    println!("    ==== Primitive Value ====");
    let mut n = 0;
    let t = thread::spawn(move || {
        n += 10;
        thread::yield_now();
        let inner_handle = thread::spawn(move || {
            n += 100;
            thread::yield_now();
            println!("Inner thread n: {n:?}")
        });
        thread::yield_now();
        println!("Outer thread n: {n:?}");
        thread::yield_now();
        inner_handle
    });
    n += 1;

    t.join().and_then(|h| h.join()).unwrap();

    println!("Main  thread n: {n:?}");
}
