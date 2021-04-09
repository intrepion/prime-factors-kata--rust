pub fn prime_factors(num: i32) -> Vec<i32> {
        vec![2]
}

#[cfg(test)]
mod tests {
        use crate::prime_factors;
        #[test]
        fn prime_factors_of_two() {
                assert_eq!(prime_factors(2), [2]);
        }
}
