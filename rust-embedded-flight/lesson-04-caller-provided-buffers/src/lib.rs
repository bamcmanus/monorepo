// Lesson 4: Caller-Provided Buffers
//
// Learning Objectives:
// - Use mutable slices to write into caller-owned output storage
// - Return the number of bytes written.
// - Use `Result` for explicit buffer-capacity errors.
// - Practice allocation-free encoding with deterministic byte order.
//
// Learner-owned work:
// - Calculate the required output capacity for all input samples.
// - Return `Err(EncodeError::BufferTooSmall)` before writing anything whenthe
//   output buffer cannot hold the full encoded result.
// - Encode each `u16` sample as two big-endian bytes.
// - Return the exact number of bytes written.
// - Leave byes after the written range unchanged.
#[derive(Debug, PartialEq, Eq)]
pub enum EncodeError {
    BufferTooSmall,
}

/// Encode `u16` sensor samples into caller-provided output storage.
///
/// The caller owns both slices:
/// - `samples` is borrowed immutably because this function only reads it.
/// - `output` is borrowed mutably because this function writes encoded bytes.
///
/// Each sample should become two bytes using `u16::to_be_bytes`.
pub fn encode_samples(
    samples: &[u16],
    output: &mut [u8],
) -> Result<usize, EncodeError> {
    // protect against overflow if usize * 2 overflows
    let required_len = samples
        .len()
        .checked_mul(2)
        .ok_or(EncodeError::BufferTooSmall)?;
    if output.len() < required_len {
        return Err(EncodeError::BufferTooSmall);
    }
    for (idx, sample) in samples.iter().enumerate() {
        let bytes = sample.to_be_bytes();
        let output_idx = idx * 2;

        // output[output_idx] = bytes[0];
        // output[output_idx+1] = bytes[1];
        output[output_idx..output_idx + 2].copy_from_slice(&bytes);
    }
    Ok(required_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_succeeds_with_zero_bytes_written() {
        let mut output = [0xAA; 4];

        assert_eq!(encode_samples(&[], &mut output), Ok(0));
        assert_eq!(output, [0xAA; 4]);
    }

    #[test]
    fn exact_capacity_encodes_all_samples_big_endian() {
        let mut output = [0; 4];

        assert_eq!(encode_samples(&[0x1234, 0xABCD], &mut output), Ok(4));
        assert_eq!(output, [0x12, 0x34, 0xAB, 0xCD]);
    }

    #[test]
    fn too_small_output_returns_error() {
        let mut output = [0xAA; 3];

        assert_eq!(
            encode_samples(&[0x1234, 0x5678], &mut output),
            Err(EncodeError::BufferTooSmall)
        );
        assert_eq!(output, [0xAA; 3]);
    }

    #[test]
    fn oversized_output_leaves_tail_unchanged() {
        let mut output = [0xEE; 6];

        assert_eq!(encode_samples(&[0x0102, 0x0304], &mut output), Ok(4));
        assert_eq!(output, [0x01, 0x02, 0x03, 0x04, 0xEE, 0xEE]);
    }
}

// Learning Journal:
//
// Answer these before marking the lesson complete.
//
// 1. Which slice is immutable and which is mutable? output is mutable and
//    - samples: &[u16] is an immutable borrowed slice
//    - output: &mut [u8] is a mutable borrowed slice
//
// 2. What prevents writing past the end of `output`? I do a size check to
//    ensure I don't write outside the bounds. If it's not big enough the fn
//    returns early to prevent bounds error.
//
// 3. Why return `usize`? Largest possible size based on machine type to allow
//    Because the value is a byte count/length, and Rust slice lengths and
//    indexes use `usize`. Returning `usize` lets callers use the result
//    directly for slicing, such as `&output[..written]`
//
// 4. What behavior should callers rely on after an error? Callers can rely on
//    no partial writes / no modification on error.
