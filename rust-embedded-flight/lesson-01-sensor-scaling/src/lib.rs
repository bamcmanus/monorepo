/// Highest valid raw ADC count for the simulated 12-bit sensor.
pub const MAX_ADC_COUNTS: u16 = 4095;

/// Sensor reference voltage in millivolts at full-scale ADC input.
pub const REFERENCE_MILLIVOLTS: u16 = 3300;

/// Convert raw ADC counts into millivolts.
///
/// The implementation should first clamp `counts` to `MAX_ADC_COUNTS`, then
/// scale the clamped value into the range `0..=REFERENCE_MILLIVOLTS` using
/// integer arithmetic. Preserve Rust's normal truncating integer division
/// behavior for values that do not divide evenly.
pub fn adc_to_millivolts(_counts: u16) -> u16 {
    if _counts == 0 {
        return 0;
    }
    let clamped_count = clamp_counts(_counts);
    // Cast to u32 to allow for larger ints with the multiplication
    (clamped_count as u32 * REFERENCE_MILLIVOLTS as u32 / MAX_ADC_COUNTS as u32)
        as u16
}

/// Clamp raw ADC counts to the valid sensor range.
///
/// Values greater than `MAX_ADC_COUNTS` should return `MAX_ADC_COUNTS`; all
/// other values should be returned unchanged.
pub fn clamp_counts(_counts: u16) -> u16 {
    if _counts > MAX_ADC_COUNTS {
        return MAX_ADC_COUNTS;
    }
    _counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_counts_convert_to_zero_millivolts() {
        assert_eq!(adc_to_millivolts(0), 0);
    }

    #[test]
    fn maximum_counts_convert_to_reference_millivolts() {
        assert_eq!(adc_to_millivolts(MAX_ADC_COUNTS), REFERENCE_MILLIVOLTS);
    }

    #[test]
    fn midpoint_conversion_uses_deterministic_truncation() {
        assert_eq!(adc_to_millivolts(2048), 1650);
    }

    #[test]
    fn above_maximum_counts_are_clamped() {
        assert_eq!(clamp_counts(MAX_ADC_COUNTS + 1), MAX_ADC_COUNTS);
        assert_eq!(adc_to_millivolts(MAX_ADC_COUNTS + 1), REFERENCE_MILLIVOLTS);
    }
}
