use std::collections::HashMap;


pub fn is_valid(s: String) -> bool {
    let mut map:HashMap<char,char> = HashMap::new();
    map.insert('(',')' );
    map.insert('{','}' );
    map.insert('[',']' );

    let mut stack: Vec<char> = vec![];
    for i in s.chars() {
        match stack.last() {
            Some(c) => {
                match map.get(c) {
                    Some(l) => {
                        if l == &i {
                            stack.pop();
                        }
                        else {
                            stack.push(i);
                        }
                    },
                    None => {
                        return false;
                    }
                }
                
            },
            None => {
                stack.push(i);
            } 
        }

    }
    if stack.is_empty() {
        return true;
    }
    false
}

mod tests {
    use  super::*;

    #[test]
    fn test1(){
        assert_eq!(is_valid(String::from("()")), true)
    }

    #[test]
    fn test2(){
        assert_eq!(is_valid(String::from("()][{}")), false)
    }
}