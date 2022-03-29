/// Clamp number to min/max
/// From [`num`] crate
#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

/// Clamp number to min
/// From [`num`] crate
#[inline]
pub fn clamp_min<T: PartialOrd>(input: T, min: T) -> T {
    debug_assert!(min == min, "min must not be NAN");
    if input < min {
        min
    } else {
        input
    }
}

/// Clamp number to max
/// From [`num`] crate
#[inline]
pub fn clamp_max<T: PartialOrd>(input: T, max: T) -> T {
    debug_assert!(max == max, "max must not be NAN");
    if input > max {
        max
    } else {
        input
    }
}
