/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 *
 * https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (38.52%)
 * Likes:    7006
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 4M
 * Testcase Example:  '"abcabcbb"'
 *
 * Given a string s, find the length of the longest substring without repeating
 * characters.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not
 * a substring.
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= s.length <= 5 * 10^4
 * s consists of English letters, digits, symbols and spaces.
 *
 *
 */

// @lc code=start

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut last: [i32; 128] = [-1; 128];
        let mut left = -1;
        let mut ans = 0;
        for (i, v) in s.chars().enumerate() {
            left = max(left, last[v as usize]);
            last[v as usize] = i as i32;
            ans = max(ans, (i as i32) - left);
        }
        return ans;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        s: String,
        output: i32,
    }

    #[test]
    fn test_length_of_longest_substring() {
        let suites = vec![
            Suite {
                s: "abcabcbb".to_string(),
                output: 3,
            },
            Suite {
                s: "bbbbb".to_string(),
                output: 1,
            },
            Suite {
                s: "pwwkew".to_string(),
                output: 3,
            },
            Suite {
                s: "au".to_string(),
                output: 2,
            },
        ];

        for suite in suites {
            assert_eq!(Solution::length_of_longest_substring(suite.s), suite.output);
        }
    }
}
