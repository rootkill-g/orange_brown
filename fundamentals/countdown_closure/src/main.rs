use std::io;
use std::io::prelude::*;

fn countdown<F>(count: usize, tick: F)
where
    F: Fn(usize),
{
    for i in (1..=count).rev() {
        tick(i);
    }
}

fn main() {
    print!("Enter a counter : ");

    // print macro does not flush the stdout automatically like println macro.
    // So we have to explicitly flush the stdout.
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut scan = String::new();

    io::stdin().read_line(&mut scan).unwrap();

    println!("Starting countdown...");

    let n: usize = match scan.trim().parse::<usize>() {
        Ok(n) => n,
        Err(e) => panic!("{}", e),
    };

    countdown(n, |i| println!("tick {}...", i));
}
