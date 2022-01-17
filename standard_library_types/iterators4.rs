// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    //if num < 2 {
    //    1
    //} else {
    //    num * factorial(num - 1)
    //}

    match num {
        0 | 1 => 1,
        _ => num * factorial(num - 1)
    }

    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn factorial_of_16() {
        assert_eq!(20922789888000, factorial(16));
    }
}
