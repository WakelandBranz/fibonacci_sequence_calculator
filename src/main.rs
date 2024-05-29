// By Wakeland Branz
mod cli;
use std::env;
use num_bigint::{self, BigUint};

fn main() {
    let benchmark = cli::Benchmark::new();

    // arg[0] is options, arg[1] is the first CLI arg
    let arg: Vec<String> = env::args().collect();
    let n: u128 = cli::parse_arg(arg[1].clone()) as u128;

    println!("Calculating the {:?}th term of the fibonacci sequence with the recursive algorithm...", &n);
    //println!("{}", fib_recursive(n.clone()));
    fib_recursive(n.clone());
    let recursive_time = benchmark.get_elapsed();
    println!("Took {:?} to calculate with recursive algorithm", &recursive_time);


    println!("Calculating the {:?}th term of the fibonacci sequence with the fast doubling algorithm...", &n);
    fib(n);
    // This is not completely accurate since there's two print statements between these time calculations, but it is close enough.
    let fast_doubling_time = benchmark.get_elapsed() - recursive_time;
    println!("Took {:?} to calculate with the fast doubling algorithm", &fast_doubling_time);

    if fast_doubling_time > recursive_time {
        let diff = fast_doubling_time - recursive_time;
        let diff_micros = diff.as_micros();
        let recursive_time_micros = recursive_time.as_micros();
        let percentage_increase = (diff_micros as f64) / (recursive_time_micros as f64) * 100.0;
        println!("The fast doubling algorithm took {:?} longer to calculate, which is a {:.2}% increase in time.", diff, percentage_increase);
    }
    else {
        let diff = recursive_time - fast_doubling_time;
        let diff_micros = diff.as_micros();
        let fast_doubling_time_micros = fast_doubling_time.as_micros();
        let percentage_increase = (diff_micros as f64) / (fast_doubling_time_micros as f64) * 100.0;
        println!("The recursive algorithm took {:?} longer to calculate, which is a {:.2}% increase in time.", diff, percentage_increase);
    }
}

// Gets the first element in the tuple returned from _fib_fast_doubling 4784969
fn fib(n: u128) -> BigUint {
    _fib_fast_doubling(n).0
}

// Code translated from https://www.nayuki.io/page/fast-fibonacci-algorithms (really good guide)
// Returns the tuple (F(n), F(n+1))... (borrows and clones galore, but it works)
fn _fib_fast_doubling(n: u128) -> (BigUint, BigUint) {
    if n == 0 {
        (BigUint::from(0u8), BigUint::from(1u8))
    }
    else {
        let (a, b) = _fib_fast_doubling(n / 2);
        let c = &a * (&b * BigUint::from(2u8) - &a);
        let d = (&a * &a) + (&b * &b);
        if n % 2 == 0 {
            (c, d)
        }
        else {
            (d.clone(), c + d)
        }
    }
}

// This is never going to calculate any numbers that require a BigUInt, it's algorithmic time complexity is way too awful.
fn fib_recursive(n: u128) -> u128 {
    if n <= 1 {
        n
    }
    else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}
