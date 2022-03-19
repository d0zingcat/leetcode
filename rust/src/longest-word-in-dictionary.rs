use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut ans = "";
        let mut candidates = HashSet::new();
        candidates.insert("");

        let mut data = words.clone();
        data.sort_by(|a, b| match a.len().cmp(&b.len()) {
            Greater => Greater,
            Less => Less,
            Equal => b.cmp(a),
        });

        for word in data.iter() {
            if candidates.contains(&word[..word.len() - 1]) {
                ans = word;
                candidates.insert(word);
            }
        }
        String::from(ans)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            "world",
            Solution::longest_word(
                vec!["w", "wo", "wor", "worl", "world",]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            )
        );
        assert_eq!(
            "apple",
            Solution::longest_word(
                vec!["a", "banana", "app", "appl", "ap", "apply", "apple",]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            )
        );
    }
}
