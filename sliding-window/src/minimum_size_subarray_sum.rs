/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 *
 * https://leetcode-cn.com/problems/minimum-size-subarray-sum/description/
 *
 * algorithms
 * Medium (48.27%)
 * Likes:    1008
 * Dislikes: 0
 * Total Accepted:    279.1K
 * Total Submissions: 578.2K
 * Testcase Example:  '7\n[2,3,1,2,4,3]'
 *
 * Given an array of positive integers nums and a positive integer target,
 * return the minimal length of a contiguous subarray [numsl, numsl+1, ...,
 * numsr-1, numsr] of which the sum is greater than or equal to target. If
 * there is no such subarray, return 0 instead.
 *
 *
 * Example 1:
 *
 *
 * Input: target = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: The subarray [4,3] has the minimal length under the problem
 * constraint.
 *
 *
 * Example 2:
 *
 *
 * Input: target = 4, nums = [1,4,4]
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: target = 11, nums = [1,1,1,1,1,1,1,1]
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= target <= 10^9
 * 1 <= nums.length <= 10^5
 * 1 <= nums[i] <= 10^5
 *
 *
 *
 * Follow up: If you have figured out the O(n) solution, try coding another
 * solution of which the time complexity is O(n log(n)).
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut ans = i32::MAX;
        let mut start = 0;

        for i in 0..nums.len() {
            sum += nums[i];
            while sum >= target {
                sum -= nums[start as usize];
                ans = ans.min((i - start + 1) as i32);
                start += 1;
            }
        }
        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct Input {
        target: i32,
        nums: Vec<i32>,
    }

    struct TestCase {
        input: Input,
        output: i32,
    }

    #[test]
    fn test_min_sub_array_len() {
        let test_cases = vec![
            TestCase {
                input: Input {
                    target: 7,
                    nums: vec![2, 3, 1, 2, 4, 3],
                },
                output: 2,
            },
            TestCase {
                input: Input {
                    target: 4,
                    nums: vec![1, 4, 4],
                },
                output: 1,
            },
            TestCase {
                input: Input {
                    target: 11,
                    nums: vec![1, 1, 1, 1, 1, 1, 1, 1],
                },
                output: 0,
            },
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::min_sub_array_len(test_case.input.target, test_case.input.nums),
                test_case.output
            )
        }
    }
}
