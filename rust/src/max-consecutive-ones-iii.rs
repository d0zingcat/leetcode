impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right, mut ans, mut zeros) = (0, 0, 0_i32, 0);
        while right < nums.len() {
            if nums[right] == 0 {
                zeros += 1
            }
            while zeros > k {
                if nums[left] == 0 {
                    zeros -= 1
                }
                left += 1;
            }
            ans = ans.max((right - left + 1) as i32);
            right += 1;
        }
        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            6,
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2)
        );
        assert_eq!(
            10,
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            )
        );
    }
}
