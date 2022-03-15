pub struct Solution;
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mask = 1 << n;
        let mut max = 0;
        let mut ans = 0;
        for i in 0..mask {
            let mut val = 0;
            for j in 0..n {
                if i >> j & 1 == 1 {
                    val |= nums[j];
                }
            }
            if val > max {
                max = val;
                ans = 1;
            } else if val == max {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::count_max_or_subsets(vec![1, 3]));
        assert_eq!(7, Solution::count_max_or_subsets(vec![2, 2, 2]));
        assert_eq!(6, Solution::count_max_or_subsets(vec![3, 2, 1, 5]));
    }
}
