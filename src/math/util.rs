const MAX_ULPS_DIFF: u64 = 8;
const MAX_ABS_DIFF: f64 = f64::EPSILON * 2.0;

pub fn are_equal(a: f64, b: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        false
    } else if a.is_infinite() && b.is_infinite() {
        a == b
    } else {
        let abs_diff = f64::abs(a - b);
        if abs_diff <= MAX_ABS_DIFF {
            true
        } else {
            let a_u = a.to_bits();
            let b_u = b.to_bits();

            let ulps_diff = a_u.abs_diff(b_u);

            ulps_diff <= MAX_ULPS_DIFF
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    pub fn are_within_tolerance(a: f64, b: f64, t: f64) -> bool {
        f64::abs(a - b) <= t
    }
}
