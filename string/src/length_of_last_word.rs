/*
 * @lc app=leetcode.cn id=58 lang=rust
 *
 * [58] Length of Last Word
 *
 * https://leetcode-cn.com/problems/length-of-last-word/description/
 *
 * algorithms
 * Easy (39.30%)
 * Likes:    432
 * Dislikes: 0
 * Total Accepted:    293K
 * Total Submissions: 745.5K
 * Testcase Example:  '"Hello World"'
 *
 * Given a string s consisting of some words separated by some number of
 * spaces, return the length of the last word in the string.
 *
 * A word is a maximal substring consisting of non-space characters only.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "Hello World"
 * Output: 5
 * Explanation: The last word is "World" with length 5.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "   fly me   to   the moon  "
 * Output: 4
 * Explanation: The last word is "moon" with length 4.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "luffy is still joyboy"
 * Output: 6
 * Explanation: The last word is "joyboy" with length 6.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of only English letters and spaces ' '.
 * There will be at least one word in s.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;
        let mut index = (s.len() as i32) - 1;
        println!("index = {}", index);
        let chs: Vec<char> = s.chars().collect();
        println!("chs {:?}", chs);

        while chs[index as usize] == ' ' {
            index -= 1;
        }

        println!("index = {}", index);
        while index >= 0 && chs[index as usize] != ' ' {
            length += 1;
            index -= 1;
        }
        length
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        output: i32,
    }

    #[test]
    fn test_length_of_last_word() {
        let test_cases = vec![
            TestCase {
                input: String::from("Hello World"),
                output: 5,
            },
            TestCase {
                input: String::from("   fly me   to   the moon  "),
                output: 4,
            },
            TestCase {
                input: String::from("luffy is still joyboy"),
                output: 6,
            },
            TestCase {
                input: String::from("luffy"),
                output: 5,
            },
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::length_of_last_word(test_case.input),
                test_case.output
            );
        }
    }
}
