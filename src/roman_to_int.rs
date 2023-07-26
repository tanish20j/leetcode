use std::cmp::Ordering;

pub fn roman_map(c: &char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let mut ch = s.chars();
    let mut res: i32 = 0;
    let mut prev = roman_map(&ch.next().unwrap());
    loop {
        let letter = ch.next();
        match letter {
            Some(l) => {
                let cur = roman_map(&l);
                match cur.cmp(&prev) {
                    Ordering::Greater => {
                        res += cur - prev;
                        let letter = ch.next();
                        match letter {
                            Some(l) => {
                                prev = roman_map(&l);
                                continue;
                            }
                            None => break,
                        }
                    }
                    Ordering::Equal => {
                        res += prev;
                    }
                    Ordering::Less => {
                        res += prev;
                    }
                }
                prev = cur;
            }
            None => {
                res += prev;
                break;
            }
        }
    }

    res
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn test4() {
        assert_eq!(roman_to_int("IV".to_string()), 4);
    }
}
