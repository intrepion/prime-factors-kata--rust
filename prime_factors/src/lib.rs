pub fn prime_factors(num: i32) -> Vec<i32> {}

#[cfg(test)]
mod tests {
        #[test]
        use crate::prime_factors;
        fn prime_factors_of_two() {
                assert_eq!(prime_factors(2), [2]);
        }
}
