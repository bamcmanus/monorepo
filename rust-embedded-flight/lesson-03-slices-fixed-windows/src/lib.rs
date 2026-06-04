// Lesson 3: Slices and Fixed Windows
//
// Learning objectives:
// - Work with borrowed slices instead of owning a sample storage.
// - Iterate over fixed-size or caller-owned sample windows without allocation.
// - Return `Option` to represent empty input.
// - Compute min, max, and integer average deterministically.
//
// Learner-owned work:
// - Return `None` for an empty slice.
// - Return `Some(WindowStats)` for non-empty input.
// - Compute min, max, and average without allocating.
// - Avoid overflow when averaging large `u16` samples.

#[derive(Debug, PartialEq, Eq)]
pub struct WindowStats {
    pub min: u16,
    pub max: u16,
    pub average: u16,
}

/// Summarize a borrowed window of sensor samples.
///
/// The caller owns the sample storage. This function only borrows the slice
/// long enough to read its contents.
//pub fn summarize_window(samples: &[u16]) -> Option<WindowStats> {
//    let mut min: u16 = u16::MAX;
//    let mut max: u16 = u16::MIN;
//    let mut sum: u64 = 0;
//    for sample in samples.iter() {
//        sum += *sample as u64;
//        if *sample < min {
//            min = *sample;
//        }
//        if *sample > max {
//            max = *sample;
//        }
//    }
//    if !samples.is_empty() {
//        let average = (sum / samples.len() as u64) as u16;
//        return Some(WindowStats { min, max, average });
//    }
//
//    None
//}
// v2
pub fn summarize_window(samples: &[u16]) -> Option<WindowStats> {
    if samples.is_empty() {
        return None;
    }

    let mut min = samples[0];
    let mut max = samples[0];
    let mut sum: u64 = 0;

    for &sample in samples {
        sum += sample as u64;
        min = min.min(sample);
        max = max.max(sample);
    }

    let average = (sum / samples.len() as u64) as u16;

    Some(WindowStats { min, max, average })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_window_returns_none() {
        assert_eq!(summarize_window(&[]), None);
    }

    #[test]
    fn one_sample_window_has_same_min_max_and_average() {
        assert_eq!(
            summarize_window(&[42]),
            Some(WindowStats {
                min: 42,
                max: 42,
                average: 42,
            })
        )
    }

    #[test]
    fn multi_sample_window_reports_min_max_and_truncated_average() {
        assert_eq!(
            summarize_window(&[10, 20, 25]),
            Some(WindowStats {
                min: 10,
                max: 25,
                average: 18,
            })
        )
    }

    #[test]
    fn non_even_average_truncates_deterministically() {
        assert_eq!(
            summarize_window(&[1, 2]),
            Some(WindowStats {
                min: 1,
                max: 2,
                average: 1,
            })
        )
    }

    #[test]
    fn max_u16_values_do_not_overflow_average() {
        assert_eq!(
            summarize_window(&[u16::MAX, u16::MAX]),
            Some(WindowStats {
                min: u16::MAX,
                max: u16::MAX,
                average: u16::MAX,
            })
        )
    }

    #[test]
    fn average_stays_between_min_and_max() {
        let stats = summarize_window(&[3, 9, 12]).unwrap();

        assert!(stats.min <= stats.average);
        assert!(stats.max >= stats.average);
    }
}

// Learning Journal:
//
// Answer these before marking the lesson complete.
//
// 1. Who owns the sample storage? The caller who passes in the reference.
//
// 2. Why is a slice better than a fixed array parameter here? A slice lets the
//    function accept borrowed sample windows of different lengths without
//    owning the storage or requiring one fixed compile-time array size.
//
// 3. What overflow risk exists when summing many `u16` values? If the n u16
//    samples sum to greater than u16::MAX then we'd experience an overflow.
//
// 4. What invariant relates `min`, `average`, and `max`? For any non-empty
//    sample window, the average should be equal to or between the min and max.
//
