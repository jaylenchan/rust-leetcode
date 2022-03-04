/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] Implement strStr()
 *
 * https://leetcode-cn.com/problems/implement-strstr/description/
 *
 * algorithms
 * Easy (40.22%)
 * Likes:    1279
 * Dislikes: 0
 * Total Accepted:    572.2K
 * Total Submissions: 1.4M
 * Testcase Example:  '"hello"\n"ll"'
 *
 * Implement strStr().
 *
 * Return the index of the first occurrence of needle in haystack, or -1 if
 * needle is not part of haystack.
 *
 * Clarification:
 *
 * What should we return when needle is an empty string? This is a great
 * question to ask during an interview.
 *
 * For the purpose of this problem, we will return 0 when needle is an empty
 * string. This is consistent to C's strstr() and Java's indexOf().
 *
 *
 * Example 1:
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 * Example 2:
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 * Example 3:
 * Input: haystack = "", needle = ""
 * Output: 0
 *
 *
 * Constraints:
 *
 *
 * 0 <= haystack.length, needle.length <= 5 * 10^4
 * haystack and needle consist of only lower-case English characters.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }

        let temp_str = haystack.clone();
        let vec_str: Vec<_> = temp_str.split(&needle).collect();
        if vec_str.len() == 1 {
            return -1;
        }

        let first_item = vec_str[0];
        let output = first_item.len();

        output as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct Input {
        haystack: String,
        needle: String,
    }

    struct Suite {
        input: Input,
        output: i32,
    }

    #[test]
    fn test_str_str() {
        let suites = vec![
            Suite {
                input: Input {
                    haystack: "hello".to_string(),
                    needle: "ll".to_string(),
                },
                output: 2,
            },
            Suite {
                input: Input {
                    haystack: "aaaaa".to_string(),
                    needle: "bba".to_string(),
                },
                output: -1,
            },
            Suite {
                input: Input {
                    haystack: "".to_string(),
                    needle: "".to_string(),
                },
                output: 0,
            },
        ];

        for suite in suites {
            assert_eq!(
                Solution::str_str(suite.input.haystack, suite.input.needle),
                suite.output
            )
        }
    }
}
