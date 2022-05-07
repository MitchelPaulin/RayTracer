pub const EPSILON: f32 = 0.00001;
pub fn f32_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f32_eq_test() {
        assert!(f32_eq(0.0, 0.0));
        assert!(!f32_eq(0.01, 0.015));
        assert!(f32_eq(1.0 * 2.0 / 2.0, 1.0));
    }
}
