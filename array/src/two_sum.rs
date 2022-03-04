/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] Two Sum
 *
 * https://leetcode-cn.com/problems/two-sum/description/
 *
 * algorithms
 * Easy (52.35%)
 * Likes:    13590
 * Dislikes: 0
 * Total Accepted:    3M
 * Total Submissions: 5.8M
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * Given an array of integers nums and an integer target, return indices of the
 * two numbers such that they add up to target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * You can return the answer in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= nums.length <= 10^4
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * Only one valid answer exists.
 *
 *
 *
 * Follow-up: Can you come up with an algorithm that is less than O(n^2) time
 * complexity?
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        } else if nums.len() == 1 {
            return vec![];
        } else {
            for i in 0..(&nums).len() {
                let num1 = nums.get(i).unwrap();
                for j in i + 1..(&nums).len() {
                    let num2 = nums.get(j).unwrap();
                    if num1 + num2 == target {
                        return vec![i as i32, j as i32];
                    }
                }
            }

            return vec![];
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        target: i32,
        output: Vec<i32>,
    }

    #[test]
    fn test_two_sum() {
        let test_cases = vec![
            TestCase {
                input: vec![2, 7, 11, 15],
                target: 9,
                output: vec![0, 1],
            },
            TestCase {
                input: vec![3, 2, 4],
                target: 6,
                output: vec![1, 2],
            },
            TestCase {
                input: vec![3, 3],
                target: 6,
                output: vec![0, 1],
            },
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::two_sum(test_case.input, test_case.target),
                test_case.output
            );
        }
    }
}
