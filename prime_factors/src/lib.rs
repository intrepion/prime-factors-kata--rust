pub struct PrimeFactorsIterator {
        num: i32,
        candidate: i32,
}

impl PrimeFactorsIterator {
        pub fn new(num: i32) -> PrimeFactorsIterator {
                PrimeFactorsIterator { num, candidate: 2 }
        }
}

impl Iterator for PrimeFactorsIterator {
        type Item = i32;

        fn next(&mut self) -> Option<i32> {
                while self.num > 1 {
                        while self.num % self.candidate == 0 {
                                self.num /= self.candidate;
                                return Some(self.candidate);
                        }
                        self.candidate += 1;
                }
                None
        }
}

pub fn prime_factors(num: i32) -> Vec<i32> {
        PrimeFactorsIterator::new(num).collect()
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
