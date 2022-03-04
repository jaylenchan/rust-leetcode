/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] Valid Parentheses
 *
 * https://leetcode-cn.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (44.54%)
 * Likes:    3034
 * Dislikes: 0
 * Total Accepted:    956.8K
 * Total Submissions: 2.1M
 * Testcase Example:  '"()"'
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 *
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: s = "(]"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of parentheses only '()[]{}'.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 || s.len() % 2 != 0 {
            return false;
        }
        let s = s.chars();
        let mut vec: Vec<char> = vec![];
        let mut valid = Box::new(false);
        for ch in s {
            match ch {
                '(' | '[' | '{' => vec.push(ch),
                ')' => match vec.pop() {
                    Some(parenthese) if parenthese == '(' => {
                        *valid = true;
                    }
                    _ => {
                        *valid = false;
                        return *valid;
                    }
                },
                ']' => match vec.pop() {
                    Some(parenthese) if parenthese == '[' => {
                        *valid = true;
                    }
                    _ => {
                        *valid = false;
                        return *valid;
                    }
                },
                '}' => match vec.pop() {
                    Some(parenthese) if parenthese == '{' => {
                        *valid = true;
                    }
                    _ => {
                        *valid = false;
                        return *valid;
                    }
                },
                _ => {
                    *valid = false;
                    return *valid;
                }
            }
        }
        if vec.len() > 0 {
            *valid = false;
        }
        *valid
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        input: String,
        output: bool,
    }

    #[test]
    fn test_is_valid() {
        let suties = [
            Suite {
                input: String::from("()"),
                output: true,
            },
            Suite {
                input: String::from("()[]{}"),
                output: true,
            },
            Suite {
                input: String::from("(]"),
                output: false,
            },
            Suite {
                input: String::from("({{{}}}}))"),
                output: false,
            },
            Suite {
                input: String::from("{{{}"),
                output: false,
            },
        ];

        for suite in suties {
            assert_eq!(Solution::is_valid(suite.input), suite.output);
        }
    }
}
