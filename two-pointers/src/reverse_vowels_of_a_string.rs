/*
 * @lc app=leetcode.cn id=345 lang=rust
 *
 * [345] Reverse Vowels of a String
 *
 * https://leetcode-cn.com/problems/reverse-vowels-of-a-string/description/
 *
 * algorithms
 * Easy (54.05%)
 * Likes:    240
 * Dislikes: 0
 * Total Accepted:    120.4K
 * Total Submissions: 222.7K
 * Testcase Example:  '"hello"'
 *
 * Given a string s, reverse only all the vowels in the string and return it.
 *
 * The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both
 * cases.
 *
 *
 * Example 1:
 * Input: s = "hello"
 * Output: "holle"
 * Example 2:
 * Input: s = "leetcode"
 * Output: "leotcede"
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 3 * 10^5
 * s consist of printable ASCII characters.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_vowels(s: String) -> String {
        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut chs: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = s.len() as i32 - 1;

        while left <= right {
            let cur_left_char = chs[left as usize];
            let cur_right_char = chs[right as usize];

            if !vowels.contains(&cur_left_char) {
                left += 1;
            }
            if !vowels.contains(&cur_right_char) {
                right -= 1;
            }

            if vowels.contains(&cur_left_char) && vowels.contains(&cur_right_char) {
                chs[left as usize] = cur_right_char;
                chs[right as usize] = cur_left_char;
                left += 1;
                right -= 1;
            }
        }

        let mut output = String::new();
        output.extend(chs);

        output
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        output: String,
    }

    #[test]
    fn test_reverse_vowels() {
        let test_cases = vec![
            TestCase {
                input: "hello".to_string(),
                output: "holle".to_string(),
            },
            TestCase {
                input: "leetcode".to_string(),
                output: "leotcede".to_string(),
            },
        ];

        for test_case in test_cases {
            assert_eq!(Solution::reverse_vowels(test_case.input), test_case.output)
        }
    }
}
