use censoredpi::{write_censored_digits_of_pi_inplace, write_censored_digits_of_pi_iterative};
use futures::executor::block_on;
use jemalloc_ctl::{stats, epoch};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

const PI_50K: &str = include_str!("../../pi50k.txt");

fn main() {
    epoch::advance().unwrap();

    let allocated_start = stats::allocated::read().unwrap();

    _ = block_on(write_censored_digits_of_pi_inplace(
        PI_50K,
        futures::io::sink(),
    )).unwrap();

    epoch::advance().unwrap();

    let allocated_inplace = stats::allocated::read().unwrap() - allocated_start;
    
    _ = block_on(write_censored_digits_of_pi_iterative(
        PI_50K,
        futures::io::sink(),
    )).unwrap();

    epoch::advance().unwrap();

    let allocated_iterative = stats::allocated::read().unwrap() - allocated_inplace;

    println!("{allocated_inplace} bytes for in-place, {allocated_iterative} bytes for iterative");
}
