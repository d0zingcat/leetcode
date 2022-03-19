impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for i in left..=right {
            let mut n = i;
            while n > 0 {
                if n % 10 == 0 || i % (n % 10) != 0 {
                    break;
                }
                n = n / 10;
            }
            if n == 0 {
                res.push(i)
            }
        }
        res
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22],
            Solution::self_dividing_numbers(1, 22)
        );
        assert_eq!(
            vec![48, 55, 66, 77],
            Solution::self_dividing_numbers(47, 85)
        )
    }
}
