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
        use std::{collections::HashMap, f64::INFINITY};

        if strs.len() == 1 {
            return strs[0].clone();
        }

        let mut min_len = INFINITY as usize;
        let mut vec_strs: Vec<Vec<char>> = vec![vec![]; strs.len()];
        for index in 0..strs.len() {
            let str = &strs[index];
            if min_len > str.len() {
                min_len = str.len();
            }
            vec_strs[index] = str.chars().collect();
        }

        let mut vec = vec!['_'; min_len];
        let mut hashmap: HashMap<usize, i32> = HashMap::new();
        for vec_str in &vec_strs {
            for i in 0..vec.len() {
                if vec[i] == '_' {
                    vec[i] = vec_str[i];
                    hashmap.insert(i, 1);
                } else {
                    if vec[i] == vec_str[i] {
                        hashmap.insert(i, hashmap.get(&i).unwrap() + 1);
                    } else {
                        vec[i] = vec_str[i];
                        hashmap.insert(i, hashmap.get(&i).unwrap() - 1);
                        break;
                    }
                }
            }
        }
        let mut ans: Vec<char> = vec![];
        for i in 0..vec.len() {
            if *hashmap.get(&i).unwrap() == strs.len() as i32 {
                ans.push(vec[i]);
            } else {
                break;
            }
        }
        let mut output = String::with_capacity(ans.len());
        output.extend(ans);
        output
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
