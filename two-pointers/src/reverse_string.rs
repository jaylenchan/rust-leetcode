/*
 * @lc app=leetcode.cn id=344 lang=rust
 *
 * [344] Reverse String
 *
 * https://leetcode-cn.com/problems/reverse-string/description/
 *
 * algorithms
 * Easy (78.06%)
 * Likes:    542
 * Dislikes: 0
 * Total Accepted:    475K
 * Total Submissions: 608.3K
 * Testcase Example:  '["h","e","l","l","o"]'
 *
 * Write a function that reverses a string. The input string is given as an
 * array of characters s.
 *
 * You must do this by modifying the input array in-place with O(1) extra
 * memory.
 *
 *
 * Example 1:
 * Input: s = ["h","e","l","l","o"]
 * Output: ["o","l","l","e","h"]
 * Example 2:
 * Input: s = ["H","a","n","n","a","h"]
 * Output: ["h","a","n","n","a","H"]
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^5
 * s[i] is a printable ascii character.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_string(s: &mut Vec<char>) -> Vec<char> {
        let mut left = 0;
        let mut right = s.len() as i32 - 1;

        while left <= right {
            let (left_ch, right_ch) = (s[left as usize], s[right as usize]);
            s[left as usize] = right_ch;
            s[right as usize] = left_ch;
            left += 1;
            right -= 1;
        }
        s.clone()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<char>,
        output: Vec<char>,
    }

    #[test]
    fn test_reverse_string() {
        let test_cases = vec![
            TestCase {
                input: vec!['h', 'e', 'l', 'l', 'o'],
                output: vec!['o', 'l', 'l', 'e', 'h'],
            },
            TestCase {
                input: vec!['H', 'a', 'n', 'n', 'a', 'h'],
                output: vec!['h', 'a', 'n', 'n', 'a', 'H'],
            },
            TestCase {
                input: vec!['a', 'b', 'c', 'd', 'e', 'f'],
                output: vec!['f', 'e', 'd', 'c', 'b', 'a'],
            },
        ];

        for mut test_case in test_cases {
            assert_eq!(
                Solution::reverse_string(&mut test_case.input),
                test_case.output
            )
        }
    }
}
