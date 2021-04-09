pub fn prime_factors(mut num: i32) -> Vec<i32> {
        let mut answer = vec![];
        while num != 1 {
                for candidate in 2..=num {
                        if num % candidate == 0 {
                                answer.push(candidate);
                                num = num / candidate;
                                break;
                        }
                }
        }
        answer
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

        #[test]
        fn prime_factors_of_nine() {
                assert_eq!(prime_factors(9), [3, 3]);
        }
}
