pub fn is_palindrome(x: i32) -> bool {
    let orig = x.to_string();
    let revs: String = orig.chars().rev().collect();

    orig == revs
}

pub fn is_palindrome_math(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut tmp = x;
    let mut s: i32 = 0;
    while tmp != 0 {
        s = s * 10 + tmp % 10;
        tmp /= 10;
    }
    x == s
}

mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test1() {
        print!("hello");
        assert!(is_palindrome(121))
    }

    #[test]
    fn perf_compare() {
        let start_a = Instant::now();
        is_palindrome(121);
        is_palindrome(-121);
        is_palindrome(10);
        let duration_a = start_a.elapsed();

        let start_b = Instant::now();
        is_palindrome_math(121);
        is_palindrome_math(-121);
        is_palindrome_math(10);
        let duration_b = start_b.elapsed();

        println!("Function A duration: {:?}", duration_a);
        println!("Function B duration: {:?}", duration_b);
    }
}
