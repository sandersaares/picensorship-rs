use censoredpi::write_censored_digits_of_pi_inplace;
use futures::executor::block_on;

const PI_50K: &str = include_str!("../../../pi50k.txt");

/// cargo build --release
/// valgrind --tool=massif target/release/inplace
/// ms_print massif.out.12345
fn main() {
    _ = block_on(write_censored_digits_of_pi_inplace(
        PI_50K,
        futures::io::sink(),
    ))
    .unwrap();
}
