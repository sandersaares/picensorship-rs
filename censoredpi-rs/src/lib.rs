use anyhow::Result;
use futures::AsyncWriteExt;

/// Censors any digit of Pi that is smaller than the previous digit, returning count of censored digits.
pub async fn write_censored_digits_of_pi_iterative(
    pi: &str,
    mut output: impl AsyncWriteExt + Unpin,
) -> Result<usize> {
    let prefix_str = &pi[0..2];
    let suffix_str = &pi[2..];

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
    let mut previous = b'0';

    let censored_suffix_bytes = suffix_str
        .bytes()
        .map(|c| {
            let is_smaller_than_previous = c < previous;
            previous = c;

            if is_smaller_than_previous {
                censored_count += 1;
                b'*'
            } else {
                c
            }
        })
        .collect::<Vec<_>>();

    output.write_all(prefix_str.as_bytes()).await?;
    output.write_all(censored_suffix_bytes.as_slice()).await?;

    Ok(censored_count)
}

// We perform the in-place censorship in chunks of up to 4KB for parity with the C# implementation.
const CHUNK_SIZE: usize = 4096;

/// Censors any digit of Pi that is smaller than the previous digit, returning count of censored digits.
pub async fn write_censored_digits_of_pi_inplace(
    pi: &str,
    mut output: impl AsyncWriteExt + Unpin,
) -> Result<usize> {
    // We can directly emit the "3." prefix.
    output.write_all(pi[0..2].as_bytes()).await?;

    // We operate directly on UTF-8 bytes because the digits of Pi cannot contain any non-ASCII characters,
    // so we can avoid dealing with characters altogether.

    let mut buffer = [0_u8; CHUNK_SIZE];

    let mut remaining = pi[2..].as_bytes();

    let mut censored_count = 0;

    // We keep track of the previous number as a character and just integer-compare each one.
    // Start with zero (lowest value) to ensure the first number is allowed without special cases in algorithm.
    let mut previous: u8 = b'0';

    while !remaining.is_empty() {
        let chunk_size = CHUNK_SIZE.min(remaining.len());
        let effective_buffer = &mut buffer[..chunk_size];

        effective_buffer.copy_from_slice(&remaining[..chunk_size]);
        remaining = &remaining[chunk_size..];

        censored_count += censor_pi_suffix_chunk(effective_buffer, &mut previous);
        output.write_all(effective_buffer).await?;
    }

    Ok(censored_count)
}

fn censor_pi_suffix_chunk(suffix: &mut [u8], previous: &mut u8) -> usize {
    // Business logic for consecutive suffix numbers:
    // * if the number gets bigger, we allow it.
    // * if the number is equal, we allow it.
    // * if the number gets smaller, we censor it.
    // * the first number is allowed.
    // e.g. 3.14*59*6**589*9**38*6*6**3...

    // We return how many numbers we censored.
    let mut censored_count = 0;

    for byte in suffix.iter_mut() {
        let is_smaller_than_previous = *byte < *previous;
        *previous = *byte;

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
