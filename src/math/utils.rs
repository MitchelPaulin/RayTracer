pub const EPSILON: f64 = 0.00001;
pub fn f64_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f64_eq_test() {
        assert!(f64_eq(0.0, 0.0));
        assert!(!f64_eq(0.01, 0.015));
        assert!(f64_eq(1.0 * 2.0 / 2.0, 1.0));
    }
}
