use std::time::{Instant, Duration};

pub struct Benchmark {
    pub start_time: Instant
}

impl Benchmark {
    pub fn new() -> Self {
        Benchmark {
            start_time: Instant::now(),
        }
    }

    pub fn get_elapsed(&self) -> Duration {
        return self.start_time.elapsed();
    }
}

pub fn parse_arg(arg: String) -> usize {
    match arg.parse::<usize>() {
        Ok(parsed_num) => {
            // check if we should even calculate the input in fib.rs
            if parsed_num < 1 {
                panic!("Invalid input: Parsed length is less than 1")
            }

            // conversion successful
            parsed_num
        }
        Err(e) => {
            // conversion failed
            panic!("Invalid input: Could not parse length to type 'usize'\nError: {}", e)
        }
    }
}