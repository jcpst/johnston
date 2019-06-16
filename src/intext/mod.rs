use rug::{Integer};
use rug::integer::IsPrime;

pub mod intext {
pub trait IntExt {
    fn factors(self) -> Vec<i32>;
    fn is_prime(self) -> bool;
    fn is_power_of_two(self) -> bool;
    fn gpf(self) -> i32;
}

impl IntExt for i32 {
    fn factors(self) -> Vec<i32> {
        let nums: Vec<_> = (1..=self).collect();
        let mut result: Vec<i32> = Vec::new();
        for n in nums {
            if self % n == 0 {
                result.push(n);
            }
        }

        result
    }

    fn is_prime(self) -> bool {
        let val = Integer::from(self).is_probably_prime(15);
        if let IsPrime::No = val {
            false
        } else {
            true
        }
    }

    fn is_power_of_two(self) -> bool {
        self != 0 && (self & (self - 1)) == 0
    }

    fn gpf(self) -> i32 {
        let mut result: i32 = 0;
        for n in self.factors() {
            if n.is_prime() && n > result {
                result = n;
            }
        }

        result
    }
}
}
