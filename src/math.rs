fn factors(num: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for n in 1..=num {
        if num % n == 0 {
            result.push(n);
        }
    }

    result
}

fn prime(num: i32) -> bool {
    for n in 2..num {
        if num % n == 0 {
            return false;
        }
    }

    true
}

pub(crate) fn gpf(num: i32) -> i32 {
    let mut result = 0;

    for n in factors(num) {
        if prime(n) && n > result {
            result = n;
        }
    }

    result
}
