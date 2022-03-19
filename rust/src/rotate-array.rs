impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.rotate_right(k as usize);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let a = vec![5, 6, 7, 1, 2, 3, 4];
        let mut b = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut b, 10);
        println!("{:?}", b);
        assert_eq!(a.iter().eq(b.iter()), true);
    }
}
