use anyhow::{anyhow, bail, Result};
use futures::AsyncWriteExt;

/// Censors any digit of Pi that is smaller than the previous digit, returning count of censored digits.
pub async fn write_censored_digits_of_pi_iterative(
    pi: &str,
    mut output: impl AsyncWriteExt + Unpin,
) -> Result<usize> {
    let (prefix_str, suffix_str) = pi
        .split_once('.')
        .ok_or(anyhow!("π must be in the form of '3.14159...'"))?;

    // The 3. in prefix_str is always the same, so we can just write it out.
    // The long tail in suffix_str is what we actually censor.

    // Business logic for consecutive suffix numbers:
    // * if the number gets bigger, we allow it.
    // * if the number is equal, we allow it.
    // * if the number gets smaller, we censor it.
    // * the first number is allowed.
    // e.g. 3.14*59*6**589*9**38*6*6**3...

    let mut censored_count = 0;

    // We keep track of the previous number as a character and just ASCII-compare each one.
    // Start with zero (lowest value) to ensure the first number is allowed without special cases in algorithm.
    let mut previous = '0';

    let censored_suffix = suffix_str
        .chars()
        .map(|c| {
            let is_smaller_than_previous = c < previous;
            previous = c;

            if is_smaller_than_previous {
                censored_count += 1;
                '*'
            } else {
                c
            }
        })
        .collect::<String>();

    output.write_all(prefix_str.as_bytes()).await?;
    output.write_all(".".as_bytes()).await?;
    output.write_all(censored_suffix.as_bytes()).await?;

    Ok(censored_count)
}

/// Censors any digit of Pi that is smaller than the previous digit, returning count of censored digits.
pub async fn write_censored_digits_of_pi_inplace(
    pi: &str,
    mut output: impl AsyncWriteExt + Unpin,
) -> Result<usize> {
    if !pi.starts_with("3.") {
        bail!("π must start with '3.'");
    }

    // We operate directly on UTF-8 bytes because the digits of Pi cannot contain any non-ASCII characters,
    // so we can avoid dealing with characters altogether. Perform the censorship in a temporary buffer.

    // There is no limit on the length of Pi, so we need to allocate the buffer on the heap (either that
    // or slice up the operation into smaller subsets or some third alternative - let's take easy way for now).
    let mut buffer = Vec::from(pi.as_bytes());

    // Skip the first 2 bytes because they are the "3." prefix.
    // The rest of the used buffer is the suffix.
    let suffix = &mut buffer[2..];

    let censored_count = censor_pi_suffix(suffix);

    output.write_all(buffer.as_slice()).await?;

    Ok(censored_count)
}

fn censor_pi_suffix(suffix: &mut [u8]) -> usize {
    // Business logic for consecutive suffix numbers:
    // * if the number gets bigger, we allow it.
    // * if the number is equal, we allow it.
    // * if the number gets smaller, we censor it.
    // * the first number is allowed.
    // e.g. 3.14*59*6**589*9**38*6*6**3...

    // We return how many numbers we censored.
    let mut censored_count = 0;

    // We keep track of the previous number as a character and just integer-compare each one.
    // Start with zero (lowest value) to ensure the first number is allowed without special cases in algorithm.
    let mut previous: u8 = b'0';

    for byte in suffix.iter_mut() {
        let is_smaller_than_previous = *byte < previous;
        previous = *byte;

        if is_smaller_than_previous {
            censored_count += 1;
            *byte = b'*';
        }
    }

    censored_count
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::executor::block_on;

    #[test]
    fn inplace_ok() -> Result<()> {
        let input = "3.1415926535897932384626433";
        let expected_output = "3.14*59*6**589*9**38*6*6**3";

        let mut output_buffer = Vec::new();

        let censored_count = block_on(write_censored_digits_of_pi_inplace(
            input,
            &mut output_buffer,
        ))?;

        assert_eq!(11, censored_count);

        let output_string = String::from_utf8(output_buffer)?;

        assert_eq!(output_string.as_str(), expected_output);

        Ok(())
    }

    #[test]
    fn iterative_ok() -> Result<()> {
        let input = "3.1415926535897932384626433";
        let expected_output = "3.14*59*6**589*9**38*6*6**3";

        let mut output_buffer = Vec::new();

        let censored_count = block_on(write_censored_digits_of_pi_iterative(
            input,
            &mut output_buffer,
        ))?;

        assert_eq!(11, censored_count);

        let output_string = String::from_utf8(output_buffer)?;

        assert_eq!(output_string.as_str(), expected_output);

        Ok(())
    }
}
