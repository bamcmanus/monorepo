// Lesson 2: Ownership And Copy Semantics
//
// Learning objectives:
// - See the difference between passing a value by copy and borrowing by
//   reference.
// - Understand why small scalar wrapper types are often cheap and safe to copy.
// - Practice writing APIs that let callers keep using their values after a
//   function call.
// - Compute an absolute difference without allowing unsigned integer underflow.
//
// Work to accomplish:
// - Make `newer_sample` return the current sample.
// - Make `sample_delta` return the same positive distance regardless of input
//   order.
// - Keep the public API shape unchanged: `newer_sample` takes `Sample` by
//   value, and `sample_delta` takes borrowed `&Sample` references.

// `derive` asks the compiler to generate standard trait implementations for
// this type. These traits are part of the vocabulary for this lesson:
//
// - `Copy`: values of this type are duplicated by a simple bitwise copy instead
//   of being moved. After passing a `Sample` by value, the caller can still use
//   the original variable.
// - `Clone`: provides an explicit `.clone()` operation. `Copy` requires
//   `Clone`, so both are derived together.
// - `Debug`: lets tests and assertion failures print readable values such as
//   `Sample(1250)`.
// - `PartialEq`: enables equality checks with `assert_eq!`.
// - `Eq`: marks equality as total for this type. Since `u16` has normal exact
//   equality, `Sample` can safely implement it too.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
// This is a tuple struct: a named wrapper around one unnamed field.
//
// It gives raw `u16` sensor samples a distinct type without adding runtime
// storage overhead. The field is public so the learner can read `sample.0`
// while focusing on ownership and borrowing rather than accessor methods.
pub struct Sample(pub u16);

/// Return the current sample.
///
/// `Sample` is `Copy`, so callers can pass values here and still reuse their
/// originals afterward.
pub fn newer_sample(_previous: Sample, current: Sample) -> Sample {
    // Ownership point: both parameters are passed by value. For non-`Copy`
    // types that would move ownership into this function. Because `Sample`
    // is `Copy`, these arguments are copied instead, so the caller's
    // variables remain usable after the call.
    return current;
}

/// Compute the absolute difference between two borrowed samples.
///
/// Borrowing keeps caller ownership with the caller while this function reads
/// the sample values.
pub fn sample_delta(previous: &Sample, current: &Sample) -> u16 {
    // Borrowing point: `&Sample` means this function receives shared
    // references. It can inspect the samples, but it does not own them and
    // cannot mutate them. The caller keeps ownership and can continue using
    // both samples after this function returns.
    //
    // Arithmetic point: subtracting a larger `u16` from a smaller `u16` can
    // underflow. Compare the two values before subtracting, or use another safe
    // approach that preserves the symmetric-delta requirement.
    //
    // INITIAL IMPL
    //if current.0 > previous.0 {
    //    return current.0 - previous.0;
    //}
    //return previous.0 - current.0;
    current.0.abs_diff(previous.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test describes the main behavior for `newer_sample`: the function
    // should select the current reading. The assertions after the function call
    // prove that `previous` and `current` are still usable because `Sample` is
    // `Copy`.
    #[test]
    fn newer_sample_returns_current_sample() {
        let previous = Sample(1200);
        let current = Sample(1250);

        assert_eq!(newer_sample(previous, current), current);
        assert_eq!(previous, Sample(1200));
        assert_eq!(current, Sample(1250));
    }

    // Passing `previous` and `current` into `newer_sample` does not consume
    // them from the caller's perspective. If `Sample` were not `Copy`, this
    // test would stop compiling because the values would have moved into
    // the function.
    #[test]
    fn copied_samples_remain_usable_after_function_call() {
        let previous = Sample(10);
        let current = Sample(20);

        let selected = newer_sample(previous, current);

        assert_eq!(selected, Sample(20));
        assert_eq!(previous.0 + current.0, 30);
    }

    // This test covers immutable borrowing. The `&previous` and `&current`
    // arguments let `sample_delta` read both samples without taking ownership.
    #[test]
    fn sample_delta_uses_borrowed_inputs() {
        let previous = Sample(100);
        let current = Sample(140);

        assert_eq!(sample_delta(&previous, &current), 40);
        assert_eq!(previous, Sample(100));
        assert_eq!(current, Sample(140));
    }

    // A delta is a distance, so input order should not matter. This prevents an
    // implementation that only works when the current value is greater than the
    // previous value.
    #[test]
    fn sample_delta_is_symmetric() {
        let low = Sample(100);
        let high = Sample(140);

        assert_eq!(sample_delta(&low, &high), 40);
        assert_eq!(sample_delta(&high, &low), 40);
    }

    // Boundary values force the implementation to handle the full `u16` range.
    // The result should be `u16::MAX`, not an underflowed or wrapped value.
    #[test]
    fn sample_delta_does_not_underflow_at_boundaries() {
        let minimum = Sample(0);
        let maximum = Sample(u16::MAX);

        assert_eq!(sample_delta(&minimum, &maximum), u16::MAX);
        assert_eq!(sample_delta(&maximum, &minimum), u16::MAX);
    }
}

// Learning journal:
//
// Answer these before marking the lesson complete.
//
// 1. Which values are copied? Answer: The inputs to newer_sample are copied.
//    Does the return also get copied as the scope of the current input would be
//    terminated on return so I'm assuming the params are copied and a copy of
//    current copy is returned
//
// 2. Which values are borrowed? Answer: inputs into sample_delta are borrowed
//
// 3. Why does `Sample` implement `Copy` safely? Answer: Sample only contains a
//    u16 which itself implements copy which is what makes sample safe to
//    implement copy.
//
// 4. What would change if `Sample` contained a heap-owned string? Answer: we'd
//    need to be much more careful WRT ownership and the intent of the retunred
//    value as we'd need to appropriately return references or copy's of data
//    based on intent
