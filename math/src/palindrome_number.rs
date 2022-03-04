/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] Palindrome Number
 *
 * https://leetcode-cn.com/problems/palindrome-number/description/
 *
 * algorithms
 * Easy (57.69%)
 * Likes:    1842
 * Dislikes: 0
 * Total Accepted:    922.8K
 * Total Submissions: 1.6M
 * Testcase Example:  '121'
 *
 * Given an integer x, return true if x is palindrome integer.
 *
 * An integer is a palindrome when it reads the same backward as forward.
 *
 *
 * For example, 121 is a palindrome while 123 is not.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 *
 *
 * Example 2:
 *
 *
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it
 * becomes 121-. Therefore it is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a
 * palindrome.
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= x <= 2^31 - 1
 *
 *
 *
 * Follow up: Could you solve it without converting the integer to a string?
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        let input = x.to_string();
        let x = input.chars().rev();
        let mut output = String::with_capacity(input.len());
        output.extend(x);

        input == output
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: i32,
        output: bool,
    }

    #[test]
    fn test_is_palindrome() {
        let test_cases = vec![
            TestCase {
                input: 121,
                output: true,
            },
            TestCase {
                input: -121,
                output: false,
            },
            TestCase {
                input: 10,
                output: false,
            },
        ];

        for test_case in test_cases {
            assert_eq!(Solution::is_palindrome(test_case.input), test_case.output);
        }
    }
}
