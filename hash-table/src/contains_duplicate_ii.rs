/*
 * @lc app=leetcode.cn id=219 lang=rust
 *
 * [219] Contains Duplicate II
 *
 * https://leetcode-cn.com/problems/contains-duplicate-ii/description/
 *
 * algorithms
 * Easy (44.44%)
 * Likes:    455
 * Dislikes: 0
 * Total Accepted:    167K
 * Total Submissions: 375.8K
 * Testcase Example:  '[1,2,3,1]\n3'
 *
 * Given an integer array nums and an integer k, return true if there are two
 * distinct indices i and j in the array such that nums[i] == nums[j] and abs(i
 * - j) <= k.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1], k = 3
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,0,1,1], k = 1
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,2,3,1,2,3], k = 2
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * 0 <= k <= 10^5
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;
        let mut num_set: HashSet<i32> = HashSet::new();
        for i in 0..nums.len() {
            let num = nums[i];
            if num_set.contains(&num) {
                return true;
            }
            num_set.insert(num);
            if num_set.len() as i32 > k {
                num_set.remove(&nums[i - k as usize]);
            }
        }
        false
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct Input {
        nums: Vec<i32>,
        k: i32,
    }
    struct TestCase {
        input: Input,
        output: bool,
    }

    #[test]
    fn test_contains_nearby_duplicate() {
        let mut test_cases = vec![
            TestCase {
                input: Input {
                    nums: vec![1, 2, 3, 1],
                    k: 3,
                },
                output: true,
            },
            TestCase {
                input: Input {
                    nums: vec![1, 0, 1, 1],
                    k: 1,
                },
                output: true,
            },
            TestCase {
                input: Input {
                    nums: vec![1, 2, 3, 1, 2, 3],
                    k: 2,
                },
                output: false,
            },
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::contains_nearby_duplicate(test_case.input.nums, test_case.input.k),
                test_case.output
            )
        }
    }
}
