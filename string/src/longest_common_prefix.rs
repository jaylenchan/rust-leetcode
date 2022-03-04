/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] Longest Common Prefix
 *
 * https://leetcode-cn.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (41.87%)
 * Likes:    2073
 * Dislikes: 0
 * Total Accepted:    747.3K
 * Total Submissions: 1.8M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * Write a function to find the longest common prefix string amongst an array
 * of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 *
 * Example 1:
 *
 *
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] consists of only lower-case English letters.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        
        todo!()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<String>,
        output: String,
    }

    #[test]
    fn test_longest_common_prefix() {
        let test_cases = vec![
            TestCase {
                input: vec![
                    String::from("flower"),
                    String::from("flow"),
                    String::from("flight"),
                ],
                output: String::from("fl"),
            },
            TestCase {
                input: vec![
                    String::from("dog"),
                    String::from("racecar"),
                    String::from("car"),
                ],
                output: String::from(""),
            },
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::longest_common_prefix(test_case.input),
                test_case.output
            );
        }
    }
}
