pub fn prime_factors(num: i32) -> Vec<i32> {
        vec![num]
}

#[cfg(test)]
mod tests {
        use crate::prime_factors;

        #[test]
        fn prime_factors_of_two() {
                assert_eq!(prime_factors(2), [2]);
        }

        #[test]
        fn prime_factors_of_three() {
                assert_eq!(prime_factors(3), [3]);
        }

        #[test]
        fn prime_factors_of_four() {
                assert_eq!(prime_factors(4), [2, 2]);
        }
}
