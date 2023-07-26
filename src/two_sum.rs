pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (pos, val) in nums.iter().enumerate() {
        let b: i32 = target - val;
        for (pos1, val1) in nums.iter().skip(pos + 1).enumerate() {
            if val1 == &b {
                return vec![pos as i32, (pos1 + pos + 1) as i32];
            }
        }
    }
    return vec![0];
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
