/*
 * @lc app=leetcode.cn id=125 lang=rust
 *
 * [125] Valid Palindrome
 *
 * https://leetcode-cn.com/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (47.03%)
 * Likes:    490
 * Dislikes: 0
 * Total Accepted:    331K
 * Total Submissions: 703.7K
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * A phrase is a palindrome if, after converting all uppercase letters into
 * lowercase letters and removing all non-alphanumeric characters, it reads the
 * same forward and backward. Alphanumeric characters include letters and
 * numbers.
 *
 * Given a string s, return true if it is a palindrome, or false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "A man, a plan, a canal: Panama"
 * Output: true
 * Explanation: "amanaplanacanalpanama" is a palindrome.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: s = " "
 * Output: true
 * Explanation: s is an empty string "" after removing non-alphanumeric
 * characters.
 * Since an empty string reads the same forward and backward, it is a
 * palindrome.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 2 * 10^5
 * s consists only of printable ASCII characters.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let chs: Vec<char> = s.chars().collect();
        let mut left: i32 = 0;
        let mut right: i32 = s.len() as i32 - 1;

        if s == " " {
            return true;
        }

        while left <= right {
            let cur_left_ch = chs[left as usize];
            let cur_right_ch = chs[right as usize];

            if !cur_left_ch.is_ascii_alphanumeric() {
                left += 1;
                continue;
            }
            if !cur_right_ch.is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }

            if cur_left_ch.to_ascii_lowercase() == cur_right_ch.to_ascii_lowercase() {
                left += 1;
                right -= 1;
            } else {
                return false;
            }
        }

        return true;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        output: bool,
    }

    #[test]
    fn test_is_palindrome() {
        let test_cases = vec![
            TestCase {
                input: "A man, a plan, a canal: Panama".to_string(),
                output: true,
            },
            TestCase {
                input: "race a car".to_string(),
                output: false,
            },
            TestCase {
                input: " ".to_string(),
                output: true,
            },
        ];

        for test_case in test_cases {
            assert_eq!(Solution::is_palindrome(test_case.input), test_case.output)
        }
    }
}
