// Lesson 5: Structs, Methods, and Newtypes
//
// Learning Objectives:
// - Use tuple structs as lightweight newtypes.
// - Implement methods on small domain types.
// - Prevent accidental mixing of raw counts, millivolts, and actuator commands.
// - Keep validation near construction
//
// Learner-owned work:
// - Make `ActuatorPercent::new` reject values greater than 100.
// - Add simple value-reading methods where helpful.
// - Convert ADC counts into millivolts using the newtype wrappers.
// - Keep the implementation integer-only, panic-free, and allocation free.

pub const MAX_ADC_COUNTS: u16 = 4095;
pub const REFERENCE_MILLIVOLTS: u16 = 3300;

// Tuple structs create distinct types around simple values. These are not type
// aliases: `AdcCounts`, `Millivolts`, and `ActuatorPercent` cannot be freely
// mixed even though they contain integer fields.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AdcCounts(pub u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Millivolts(pub u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActuatorPercent(u8);

impl ActuatorPercent {
    /// Construct an actuator percentage only when the value is in range.
    ///
    /// Returning `Option<Self>` teaches fallible construction without needing a
    /// full error taxonomy yet. `None` means the requested percentage is
    /// invalid.
    pub fn new(value: u8) -> Option<Self> {
        if value > 100 {
            return None;
        }
        Some(Self(value))
    }

    /// Read the wrapped percentage value.
    pub fn value(self) -> u8 {
        self.0
    }
}

/// Convert raw ADC counts into millivolts
///
/// This function intentionally accepts `AdcCounts` and returns `Millivolts` so
/// callers cannot accidentally pass an actuator percentage into a
/// sensor-scaling function.
pub fn counts_to_millivolts(counts: AdcCounts) -> Millivolts {
    // LESS IDIOMATIC
    //let c: u16;
    //if _counts.0 > MAX_ADC_COUNTS {
    //    c = MAX_ADC_COUNTS;
    //} else {
    //    c = _counts.0;
    //}

    // clamp to max
    let clamped = counts.0.min(MAX_ADC_COUNTS);

    // convert the value
    let mv =
        (clamped as u32 * REFERENCE_MILLIVOLTS as u32) / MAX_ADC_COUNTS as u32;

    Millivolts(mv as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_actuator_percent_is_constructed() {
        assert_eq!(ActuatorPercent::new(0), Some(ActuatorPercent(0)));
        assert_eq!(ActuatorPercent::new(42), Some(ActuatorPercent(42)));
        assert_eq!(ActuatorPercent::new(100), Some(ActuatorPercent(100)));
    }

    #[test]
    fn invalid_actuator_percent_is_rejected() {
        assert_eq!(ActuatorPercent::new(101), None);
        assert_eq!(ActuatorPercent::new(u8::MAX), None);
    }

    #[test]
    fn actuator_value_method_returns_inner_value() {
        let percent = ActuatorPercent::new(75);

        assert_eq!(percent.map(ActuatorPercent::value), Some(75));
    }

    #[test]
    fn zero_counts_convert_to_zero_millivolts() {
        assert_eq!(counts_to_millivolts(AdcCounts(0)), Millivolts(0));
    }

    #[test]
    fn maximum_counts_convert_to_reference_millivolts() {
        assert_eq!(
            counts_to_millivolts(AdcCounts(MAX_ADC_COUNTS)),
            Millivolts(REFERENCE_MILLIVOLTS)
        );
    }

    #[test]
    fn above_maximum_counts_are_clamped_before_conversion() {
        assert_eq!(
            counts_to_millivolts(AdcCounts(MAX_ADC_COUNTS + 1)),
            Millivolts(REFERENCE_MILLIVOLTS)
        );
    }

    #[test]
    fn midpoint_conversion_uses_integer_truncation() {
        assert_eq!(counts_to_millivolts(AdcCounts(2048)), Millivolts(1650));
    }
}

// Learning journal:
//
// Answer these before marking the lesson complete.
//
// 1. What bug can newtypes prevent? Newtypes prevent accidentally mixing values
//    that have the same underlying representation but different meanings, such
//    as raw ADC counts, millivolts, and actuator percentages.
//
// 2. Which constructors can fail? `ActuatorPercent::new` can fail because not
//    every `u8` is a valid percentage. Values above 100 return `None`.
//
// 3. Is the inner field public, and what does that imply? `AdcCounts` and
//    `Millivolts` have public inner fields, so callers can construct and read
//    them directly. `ActuatorPercent` has a private inner field, so external
//    callers must use `ActuatorPercent::new`, which preserves the valid
//    percentage invariant.
//
// 4. When is a tuple struct better than a type alias? A tuple struct is better
//    when you want a distinct compile-time type. A type alias is only another
//    name for the same underlying type, so it does not prevent accidental
//    mixing.
